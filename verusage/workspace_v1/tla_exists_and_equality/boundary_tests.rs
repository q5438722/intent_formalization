use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions from target file ===

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

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn tla_exists_proved_by_witness<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>, witness_a: A)
    requires a_to_p(witness_a).satisfied_by(ex),
    ensures tla_exists(a_to_p).satisfied_by(ex),
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

// Test 1: Call tla_exists_proved_by_witness with a witness that does NOT satisfy the predicate.
// a_to_p(0) evaluates to TempPred(|ex| false), so the requires clause is violated.
// SHOULD FAIL
proof fn test_boundary_invalid_witness()
{
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| a == 42);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };
    // a_to_p(0).satisfied_by(ex) = (0 == 42) = false — violates requires
    tla_exists_proved_by_witness::<int, int>(ex, a_to_p, 0int);
}

// Test 2: Call temp_pred_equality with only one direction of entailment.
// p = always-false, q = always-true: p.entails(q) holds but q.entails(p) does not.
// SHOULD FAIL
proof fn test_boundary_one_direction_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    // p.entails(q): forall|ex| false ==> true = true  ✓
    // q.entails(p): forall|ex| true ==> false = false  ✗ — violates second requires
    temp_pred_equality::<int>(p, q);
}

// Test 3: Call temp_pred_equality with contradictory predicates where the first
// direction of entailment fails (always-true does NOT entail always-false).
// SHOULD FAIL
proof fn test_boundary_contradictory_predicates()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    // p.entails(q): forall|ex| true ==> false = false  ✗ — violates first requires
    temp_pred_equality::<int>(p, q);
}

}
