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

proof fn tla_exists_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_exists(|a: A| a_to_p(a).or(q)) == tla_exists(a_to_p).or(q),
{
    let a_to_p_or_q = |a: A| a_to_p(a).or(q);
    assert forall |ex| #[trigger] (tla_exists(a_to_p).or(q)).satisfied_by(ex)
    implies tla_exists(a_to_p_or_q).satisfied_by(ex) by {
        if !q.satisfied_by(ex) {
            let witness_a = tla_exists_choose_witness::<T, A>(ex, a_to_p);
            tla_exists_proved_by_witness::<T, A>(ex, a_to_p_or_q, witness_a);
        } else {
            assert(a_to_p_or_q(arbitrary()).satisfied_by(ex));
        }
    };
    temp_pred_equality::<T>(tla_exists(|a: A| a_to_p(a).or(q)), tla_exists(a_to_p).or(q));
}

// =====================================================================
// CORRECTNESS TESTS — Consolidated from boundary, behavioral mutation,
// and logical adversarial test categories.
// All tests SHOULD FAIL verification if the specification is correct.
// =====================================================================

// ======================== BOUNDARY TESTS ========================

// BT-1: Violate requires of tla_exists_proved_by_witness — witness doesn't satisfy predicate
// SHOULD FAIL
proof fn test_boundary_false_witness()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };
    tla_exists_proved_by_witness::<int, bool>(ex, always_false, true); // SHOULD FAIL
}

// BT-2: Violate requires of temp_pred_equality — only one direction of entailment
// SHOULD FAIL
proof fn test_boundary_one_way_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    temp_pred_equality::<int>(p, q); // SHOULD FAIL
}

// BT-3: Assert existence from always-false predicate
// SHOULD FAIL
proof fn test_boundary_existence_from_always_false()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };
    assert(tla_exists(always_false).satisfied_by(ex)); // SHOULD FAIL
}

// ==================== BEHAVIORAL MUTATION TESTS ====================

// BM-1: Mutate main theorem — drop .or(q) from RHS
// SHOULD FAIL
proof fn test_mutation_drop_or_from_rhs()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    tla_exists_or_equality::<int, bool>(a_to_p, q);
    assert(tla_exists(|b: bool| a_to_p(b).or(q)) == tla_exists(a_to_p)); // SHOULD FAIL
}

// BM-2: Mutate by equating tla_exists(p) with tla_exists(|a| p(a).or(q))
// SHOULD FAIL
proof fn test_mutation_reverse_strict_subset()
{
    let a_to_p: spec_fn(int) -> TempPred<int> = |i: int|
        TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == i && i == 42);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 99);
    assert(tla_exists(a_to_p) == tla_exists(|i: int| a_to_p(i).or(q))); // SHOULD FAIL
}

// BM-3: Assert valid(tla_exists(always_false)) — false predicate cannot be valid
// SHOULD FAIL
proof fn test_mutation_false_pred_valid()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    assert(valid(tla_exists(always_false))); // SHOULD FAIL
}

// ======================== LOGICAL TESTS ========================

// LT-1: Derive equality of clearly different predicates (no mutual entailment)
// SHOULD FAIL
proof fn test_logical_unrelated_equality()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 1);
    assert(p == q); // SHOULD FAIL
}

// LT-2: Assert tla_exists distributes over implies (not valid in temporal logic)
// SHOULD FAIL
proof fn test_logical_exists_distributes_over_implies()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| b && (ex.nat_to_state)(0) == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 1);
    assert(
        tla_exists(|b: bool| a_to_p(b).implies(q))
        == tla_exists(a_to_p).implies(q)
    ); // SHOULD FAIL
}

// LT-3: Existence does NOT imply a specific non-witness satisfies the predicate
// SHOULD FAIL
proof fn test_logical_existence_implies_arbitrary_witness()
{
    let a_to_p: spec_fn(int) -> TempPred<int> = |i: int|
        TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == i);
    let ex = Execution::<int> { nat_to_state: |n: nat| 42int };
    assert(a_to_p(0int).satisfied_by(ex)); // SHOULD FAIL
}

// LT-4: Structural or-idempotence without proof (p.or(p) == p needs axiom)
// SHOULD FAIL
proof fn test_logical_or_idempotent_without_proof()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| b);
    let e = tla_exists(a_to_p);
    assert(e.or(e) == e); // SHOULD FAIL
}

// LT-5: Cross-function misuse — choose_witness outside recommends scope
// SHOULD FAIL
proof fn test_logical_choose_witness_without_existence()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let ex = Execution::<int> { nat_to_state: |n: nat| 0int };
    let w = tla_exists_choose_witness::<int, bool>(ex, always_false);
    assert(always_false(w).satisfied_by(ex)); // SHOULD FAIL
}

}
