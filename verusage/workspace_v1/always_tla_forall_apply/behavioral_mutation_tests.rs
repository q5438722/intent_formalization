use vstd::prelude::*;

fn main() {}

verus! {

// ── Definitions (from source) ──────────────────────────────────────────

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn suffix(self, pos: nat) -> Self {
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
}

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {
    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred { pred: pred }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall|a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall|ex: Execution<T>| temp_pred.satisfied_by(ex)
}

pub proof fn always_tla_forall_apply<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
    a: A,
)
    requires
        spec.entails(always(tla_forall(a_to_p))),
    ensures
        spec.entails(always(a_to_p(a))),
{
    entails_preserved_by_always(tla_forall(a_to_p), a_to_p(a));
    entails_trans(spec, always(tla_forall(a_to_p)), always(a_to_p(a)));
}

#[verifier::external_body]
pub proof fn entails_trans<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures
        p.entails(r),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn entails_preserved_by_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
    ensures
        always(p).entails(always(q)),
{
    unimplemented!()
}

// ── Behavioral Mutation Tests ──────────────────────────────────────────

// SHOULD FAIL
// Test 1: Mutate conclusion to claim UNIVERSAL validity instead of entailment.
// The theorem says spec.entails(always(a_to_p(a))), but we assert valid(always(a_to_p(a))).
// valid(P) means P holds on ALL executions, not just those satisfying spec.
proof fn test_mutate_entails_to_valid(
    spec: TempPred<int>,
    a_to_p: spec_fn(int) -> TempPred<int>,
)
    requires
        spec.entails(always(tla_forall(a_to_p))),
{
    always_tla_forall_apply::<int, int>(spec, a_to_p, 0int);
    // Correct: spec.entails(always(a_to_p(0int)))
    // Mutated (too strong): valid(always(a_to_p(0int)))
    assert(valid(always(a_to_p(0int))));
}

// SHOULD FAIL
// Test 2: Reverse the entailment direction.
// The theorem gives spec.entails(always(a_to_p(a))), but we assert the converse.
proof fn test_reverse_entailment_direction(
    spec: TempPred<int>,
    a_to_p: spec_fn(int) -> TempPred<int>,
)
    requires
        spec.entails(always(tla_forall(a_to_p))),
{
    always_tla_forall_apply::<int, int>(spec, a_to_p, 0int);
    // Mutated: reverse the entailment direction
    assert(always(a_to_p(0int)).entails(spec));
}

// SHOULD FAIL
// Test 3: Claim that a_to_p(a) self-entails its own always (single-shot implies forever).
// This is not a consequence of the specification.
proof fn test_single_shot_implies_always(
    spec: TempPred<int>,
    a_to_p: spec_fn(int) -> TempPred<int>,
)
    requires
        spec.entails(always(tla_forall(a_to_p))),
{
    always_tla_forall_apply::<int, int>(spec, a_to_p, 0int);
    // Unwarranted claim: P(a) entails always(P(a))
    assert(a_to_p(0int).entails(always(a_to_p(0int))));
}

}
