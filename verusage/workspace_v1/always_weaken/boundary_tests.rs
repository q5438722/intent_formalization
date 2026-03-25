use vstd::prelude::*;

fn main() {}

verus!{

// ══════════════════════════════════════════════════════════════
// Definitions (from target: always_weaken.rs)
// ══════════════════════════════════════════════════════════════

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
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn entails_preserved_by_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures always(p).entails(always(q)),
{
    unimplemented!()
}

pub proof fn always_weaken<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(p.implies(q)),
        spec.entails(always(p)),
    ensures spec.entails(always(q)),
{
    entails_preserved_by_always::<T>(p, q);
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies always(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, always(p));
        implies_apply::<T>(ex, always(p), always(q));
    };
}

// ══════════════════════════════════════════════════════════════
// BOUNDARY TESTS — violate preconditions of always_weaken
// ══════════════════════════════════════════════════════════════

// BOUNDARY TEST 1: Missing valid(p.implies(q)) precondition
// Without the implication, weakening should not be possible.
// SHOULD FAIL
proof fn boundary_test_1_missing_implies<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(always(p)),
        // MISSING: valid(p.implies(q))
    ensures spec.entails(always(q)),
{
    always_weaken::<T>(spec, p, q);
}

// BOUNDARY TEST 2: Missing spec.entails(always(p)) precondition
// Without the entailment premise, weakening should not be possible.
// SHOULD FAIL
proof fn boundary_test_2_missing_entails<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(p.implies(q)),
        // MISSING: spec.entails(always(p))
    ensures spec.entails(always(q)),
{
    always_weaken::<T>(spec, p, q);
}

// BOUNDARY TEST 3: Reversed implication direction
// Providing valid(q → p) instead of valid(p → q) should not suffice.
// SHOULD FAIL
proof fn boundary_test_3_reversed_implies<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(q.implies(p)),  // WRONG DIRECTION
        spec.entails(always(p)),
    ensures spec.entails(always(q)),
{
    always_weaken::<T>(spec, p, q);
}

}
