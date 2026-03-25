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

// ========== Behavioral Mutation Tests ==========
// These start from valid inputs but mutate expected outputs/relations.
// All tests SHOULD FAIL verification.

// Test 1: Mutate the main theorem — drop .or(q) from the RHS
// Assert: tla_exists(|a| a_to_p(a).or(q)) == tla_exists(a_to_p)
// This drops q from the result, which is WRONG when q is satisfiable independently.
// SHOULD FAIL
proof fn test_mutation_drop_or_from_rhs()
{
    let a_to_p: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);

    // Call the real theorem to get the correct equality
    tla_exists_or_equality::<int, bool>(a_to_p, q);

    // Now assert the mutated version: tla_exists(|a| p(a).or(q)) == tla_exists(p)
    // Since p(a) is always false but q is always true,
    // LHS = tla_exists(|a| false.or(true)) = tla_exists(|a| true) which is valid
    // RHS = tla_exists(|a| false) which is never satisfied
    // So they should NOT be equal
    assert(tla_exists(|b: bool| a_to_p(b).or(q)) == tla_exists(a_to_p)); // SHOULD FAIL
}

// Test 2: Mutate by reversing the equality direction logically
// Assert: tla_exists(a_to_p) == tla_exists(|a| a_to_p(a).or(q))
// when a_to_p is strict subset of a_to_p.or(q)
// The semantics differ: left side is strictly weaker
// SHOULD FAIL
proof fn test_mutation_reverse_strict_subset()
{
    let a_to_p: spec_fn(int) -> TempPred<int> = |i: int|
        TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == i && i == 42);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 99);

    // tla_exists(a_to_p) is satisfied iff state(0) == 42 (for witness i=42)
    // tla_exists(|i| a_to_p(i).or(q)) is satisfied iff state(0) == 42 OR state(0) == 99
    // These are NOT equal (e.g., execution with state(0) == 99 satisfies RHS but not LHS without or)
    // But the real theorem says they're equal to tla_exists(a_to_p).or(q), not tla_exists(a_to_p)
    assert(tla_exists(a_to_p) == tla_exists(|i: int| a_to_p(i).or(q))); // SHOULD FAIL
}

// Test 3: Assert that valid(tla_exists(always_false_pred)) holds
// For a predicate that is never satisfied, tla_exists should NOT be valid
// SHOULD FAIL
proof fn test_mutation_false_pred_valid()
{
    let always_false: spec_fn(bool) -> TempPred<int> = |b: bool|
        TempPred::<int>::new(|ex: Execution<int>| false);

    // tla_exists of always-false should itself be always-false, hence NOT valid
    assert(valid(tla_exists(always_false))); // SHOULD FAIL
}

}
