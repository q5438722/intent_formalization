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

// ========== Logical Tests ==========
// These test properties NOT explicitly guaranteed by the specification:
// determinism, stronger inequalities, structural assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: Try to derive equality of clearly different predicates
// without establishing mutual entailment
// SHOULD FAIL - no basis for equality without proving mutual entailment
proof fn test_logical_unrelated_equality()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 1);

    // p and q are clearly different predicates (state(0)==0 vs state(0)==1)
    // They are NOT mutually entailing, so we cannot derive equality
    assert(p == q); // SHOULD FAIL
}

// Test 2: Assert that tla_exists distributes over implies
// This is NOT a valid law of temporal logic.
// tla_exists(|a| p(a).implies(q)) != tla_exists(|a| p(a)).implies(q) in general
// SHOULD FAIL
proof fn test_logical_exists_distributes_over_implies()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| b && (ex.nat_to_state)(0) == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 1);

    // tla_exists(|b| p(b).implies(q)):
    //   exists b. (b && state(0)==0) ==> (state(0)==1)
    //   For b=false, the antecedent is false, so implication is true
    //   So this is always true (valid)
    //
    // tla_exists(|b| p(b)).implies(q):
    //   (exists b. b && state(0)==0) ==> (state(0)==1)
    //   = (state(0)==0) ==> (state(0)==1)
    //   This is false when state(0)==0
    //
    // So they differ. This equality should NOT hold.
    assert(
        tla_exists(|b: bool| a_to_p(b).implies(q))
        == tla_exists(a_to_p).implies(q)
    ); // SHOULD FAIL
}

// Test 3: Assert that existence gives a SPECIFIC witness that is always valid
// tla_exists(a_to_p).satisfied_by(ex) does NOT mean a_to_p(arbitrary()).satisfied_by(ex)
// SHOULD FAIL
proof fn test_logical_existence_implies_arbitrary_witness()
{
    let a_to_p: spec_fn(int) -> TempPred<int> = |i: int|
        TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == i);
    let ex = Execution::<int> { nat_to_state: |n: nat| 42int };

    // tla_exists(a_to_p).satisfied_by(ex) is true (witness: i=42)
    // But a_to_p(0).satisfied_by(ex) is false (state(0) is 42, not 0)
    // So existence does not imply a specific non-witness satisfies the predicate
    assert(a_to_p(0int).satisfied_by(ex)); // SHOULD FAIL
}

// Test 4: Try to prove a stronger property — that or is idempotent with tla_exists
// Assert: tla_exists(a_to_p).or(tla_exists(a_to_p)) == tla_exists(a_to_p)
// While logically p ∨ p ≡ p, proving structural equality in Verus requires
// the temp_pred_equality axiom, which needs mutual entailment to be established.
// Without proof, this should fail.
// SHOULD FAIL
proof fn test_logical_or_idempotent_without_proof()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| b);
    let e = tla_exists(a_to_p);

    // p.or(p) is semantically equivalent to p, but structural equality is not free
    assert(e.or(e) == e); // SHOULD FAIL
}

// Test 5: Cross-function misuse — use tla_exists_choose_witness outside its recommends
// and try to derive contradictory facts
// SHOULD FAIL
proof fn test_logical_choose_witness_without_existence()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };

    // No witness exists, but choose still returns *some* value
    let w = tla_exists_choose_witness::<int, bool>(ex, always_false);

    // The chosen witness should NOT satisfy the predicate (since nothing does)
    // Asserting it does should fail
    assert(always_false(w).satisfied_by(ex)); // SHOULD FAIL
}

}
