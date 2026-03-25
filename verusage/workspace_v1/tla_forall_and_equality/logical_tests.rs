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

// === LOGICAL TESTS ===

// Test 1: Derive false from the axiom system.
// If the axioms (tla_forall_unfold, temp_pred_equality) are consistent,
// it should be impossible to prove false without any premises.
// SHOULD FAIL
proof fn test_logical_derive_false()
{
    assert(false);
}

// Test 2: Assert equality of extensionally different predicates
// without establishing mutual entailment.
// p (always false) and q (always true) differ on every execution.
// Without calling temp_pred_equality with both entailments, equality cannot hold.
// SHOULD FAIL
proof fn test_logical_equality_without_mutual_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    assert(p == q);
}

// Test 3: Assert execution uniqueness — a structural/global property NOT guaranteed by the spec.
// Two different executions can satisfy the same predicate; they need not be identical.
// The spec makes no determinism or uniqueness guarantees about executions.
// SHOULD FAIL
proof fn test_logical_execution_uniqueness(ex1: Execution<int>, ex2: Execution<int>, p: TempPred<int>)
    requires
        p.satisfied_by(ex1),
        p.satisfied_by(ex2),
    ensures ex1 == ex2,
{
}

}
