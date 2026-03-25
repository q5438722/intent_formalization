use vstd::prelude::*;

fn main() {}

verus! {

// ============================================================
// Base definitions (from target file)
// ============================================================

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

    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
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

pub proof fn entails_and_different_temp<T>(spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec1.entails(p),
        spec2.entails(q),
    ensures spec1.and(spec2).entails(p.and(q)),
{
    assert forall |ex| #[trigger] spec1.and(spec2).satisfied_by(ex) implies p.and(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec1, p);
        implies_apply::<T>(ex, spec2, q);
    };
}

// ============================================================
// Boundary Tests: violate preconditions / edge cases
// ============================================================

// Test 1: Missing first precondition spec1.entails(p)
// SHOULD FAIL
proof fn test_boundary_missing_first_precondition<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec2.entails(q),
    ensures spec1.and(spec2).entails(p.and(q)),
{
    entails_and_different_temp(spec1, spec2, p, q);
}

// Test 2: Missing second precondition spec2.entails(q)
// SHOULD FAIL
proof fn test_boundary_missing_second_precondition<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec1.entails(p),
    ensures spec1.and(spec2).entails(p.and(q)),
{
    entails_and_different_temp(spec1, spec2, p, q);
}

// Test 3: Missing both preconditions
// SHOULD FAIL
proof fn test_boundary_missing_both_preconditions<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    ensures spec1.and(spec2).entails(p.and(q)),
{
    entails_and_different_temp(spec1, spec2, p, q);
}

// Test 4: implies_apply called without the implication precondition
// SHOULD FAIL
proof fn test_boundary_implies_apply_no_implication<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// Test 5: implies_apply called without the antecedent precondition
// SHOULD FAIL
proof fn test_boundary_implies_apply_no_antecedent<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

}
