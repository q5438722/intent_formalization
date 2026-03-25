use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions (from target file) ===

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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// === Axioms (external_body from target file) ===

#[verifier::external_body]
proof fn tla_forall_unfold<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires tla_forall(a_to_p).satisfied_by(ex),
    ensures forall |a| #[trigger] a_to_p(a).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    unimplemented!()
}

// === BOUNDARY TESTS ===

// Test 1: Call tla_forall_unfold without satisfying its requires clause.
// p is arbitrary, so tla_forall(|a| p).satisfied_by(ex) = p.satisfied_by(ex)
// is not guaranteed for arbitrary p and ex.
// SHOULD FAIL
proof fn test_boundary_unfold_precondition_violated(ex: Execution<int>, p: TempPred<int>)
{
    let a_to_p = |a: int| p;
    tla_forall_unfold::<int, int>(ex, a_to_p);
}

// Test 2: Call temp_pred_equality with only one direction of entailment.
// p (always false) trivially entails q (always true) via vacuous implication,
// but q does NOT entail p. The second requires clause is violated.
// SHOULD FAIL
proof fn test_boundary_equality_missing_reverse()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    temp_pred_equality::<int>(p, q);
}

// Test 3: Assert valid(p) for a predicate that is not universally true.
// An arbitrary predicate p is not necessarily satisfied by all executions.
// SHOULD FAIL
proof fn test_boundary_valid_not_universal(p: TempPred<int>)
{
    assert(valid(p));
}

}
