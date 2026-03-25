use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions (from target) ==========

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

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
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
    ensures tla_exists(a_to_p).satisfied_by(ex)
{
    unimplemented!()
}

spec fn tla_exists_choose_witness<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>) -> A
    recommends exists |a| #[trigger] a_to_p(a).satisfied_by(ex),
{
    let witness = choose |a| #[trigger] a_to_p(a).satisfied_by(ex);
    witness
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

// ========== Boundary Tests ==========
// These tests violate preconditions or use edge-case inputs.
// All tests SHOULD FAIL verification.

// Test 1: Provide a witness that does NOT satisfy the predicate
// SHOULD FAIL - precondition violation: the witness doesn't satisfy a_to_p
proof fn test_boundary_false_witness()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };

    // The witness `true` does NOT satisfy `always_false(true)` because the predicate is always false
    // This violates the `requires` of tla_exists_proved_by_witness
    tla_exists_proved_by_witness::<int, bool>(ex, always_false, true); // SHOULD FAIL
}

// Test 2: Call temp_pred_equality with only one direction of entailment
// SHOULD FAIL - precondition violation: only p.entails(q) holds, not q.entails(p)
proof fn test_boundary_one_way_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);

    // false ==> true is valid, but true ==> false is not
    // So p.entails(q) holds but q.entails(p) does NOT hold
    temp_pred_equality::<int>(p, q); // SHOULD FAIL
}

// Test 3: Assert tla_exists is satisfied when no witness can exist
// SHOULD FAIL - there's no bool value that makes always_false true
proof fn test_boundary_existence_from_always_false()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };

    // No witness exists, so tla_exists should NOT be satisfied
    assert(tla_exists(always_false).satisfied_by(ex)); // SHOULD FAIL
}

}
