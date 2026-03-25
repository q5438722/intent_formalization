use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions from target file =====

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
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

pub proof fn spec_entails_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
    ensures spec.entails(tla_forall(a_to_p)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies tla_forall(a_to_p).satisfied_by(ex) by {
        assert forall |a| #[trigger] a_to_p(a).satisfied_by(ex) by {
            implies_apply::<T>(ex, spec, a_to_p(a));
        };
    };
}

// ===== BOUNDARY TESTS =====

// Test 1: Call spec_entails_tla_forall without its precondition
// The precondition requires forall |a| spec.entails(a_to_p(a)), but we provide no such guarantee.
// SHOULD FAIL
proof fn test_boundary_missing_forall_precondition<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
)
{
    spec_entails_tla_forall(spec, a_to_p); // precondition not established
}

// Test 2: Call implies_apply without proving p.satisfied_by(ex)
// Only the implication is given, not the antecedent.
// SHOULD FAIL
proof fn test_boundary_implies_apply_missing_antecedent<T>(
    ex: Execution<T>,
    p: TempPred<T>,
    q: TempPred<T>,
)
    requires
        p.implies(q).satisfied_by(ex),
        // MISSING: p.satisfied_by(ex)
{
    implies_apply(ex, p, q);
}

// Test 3: Call implies_apply without proving p.implies(q).satisfied_by(ex)
// Only the antecedent is given, not the implication.
// SHOULD FAIL
proof fn test_boundary_implies_apply_missing_implication<T>(
    ex: Execution<T>,
    p: TempPred<T>,
    q: TempPred<T>,
)
    requires
        p.satisfied_by(ex),
        // MISSING: p.implies(q).satisfied_by(ex)
{
    implies_apply(ex, p, q);
}

// Test 4: Call spec_entails_tla_forall with a spec that only entails SOME (not all) a_to_p(a).
// The precondition requires forall, not exists.
// SHOULD FAIL
proof fn test_boundary_partial_entailment<T>(
    spec: TempPred<T>,
    a_to_p: spec_fn(bool) -> TempPred<T>,
)
    requires
        spec.entails(a_to_p(true)),
        // MISSING: spec.entails(a_to_p(false))
{
    spec_entails_tla_forall::<T, bool>(spec, a_to_p);
}

}
