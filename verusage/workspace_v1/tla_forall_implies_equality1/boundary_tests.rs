use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions (copied from target) ===

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

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// Axioms (from target)
#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{ unimplemented!() }


// === BOUNDARY TESTS ===

// BT1: Use temp_pred_equality with only one direction of entailment.
// p entails q but NOT q entails p. Should not conclude p == q.
// SHOULD FAIL
proof fn bt1_one_direction_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);   // always true
    let q = TempPred::<int>::new(|ex: Execution<int>| false);  // always false
    // p does NOT entail q (true =/=> false), q entails p (false ==> true vacuously)
    // We try to assert p == q without both directions
    assert(p == q); // SHOULD FAIL: p != q
}

// BT2: Use a_to_temp_pred_equality when only one direction holds pointwise.
// SHOULD FAIL
proof fn bt2_one_direction_pointwise()
{
    let p_fn = |a: int| TempPred::<int>::new(|ex: Execution<int>| true);
    let q_fn = |a: int| TempPred::<int>::new(|ex: Execution<int>| a > 0);
    // p_fn(a) entails q_fn(a) only when a > 0, not always
    // We try to conclude they are equal without establishing bi-entailment
    assert(p_fn == q_fn); // SHOULD FAIL: not pointwise equivalent
}

// BT3: Assert equality of semantically equivalent but structurally
// different TempPreds WITHOUT invoking temp_pred_equality axiom.
// SHOULD FAIL
proof fn bt3_structural_vs_semantic_equality()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0 || (ex.nat_to_state)(0) < 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    // Semantically identical (tautology) but structurally different closures.
    // Without invoking temp_pred_equality, Verus should not conclude p == q.
    assert(p == q); // SHOULD FAIL: structural equality not automatic
}

// BT4: Try to derive equality for contradictory predicates.
// SHOULD FAIL
proof fn bt4_contradictory_predicates_equal()
{
    let tt = TempPred::<int>::new(|ex: Execution<int>| true);
    let ff = TempPred::<int>::new(|ex: Execution<int>| false);
    // true does not entail false
    assert(tt.entails(ff)); // SHOULD FAIL: true =/=> false
}

// BT5: Try using tla_forall with an empty domain claim.
// Assert that tla_forall over all ints of a non-trivial predicate equals true.
// SHOULD FAIL
proof fn bt5_forall_non_trivial_is_not_valid()
{
    let p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    // tla_forall(p) means for ALL a, (ex.nat_to_state)(0) == a. That's impossible.
    assert(valid(tla_forall(p))); // SHOULD FAIL: can't equal all ints simultaneously
}

}
