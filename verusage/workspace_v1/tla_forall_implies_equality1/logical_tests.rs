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

#[verifier::external_body]
proof fn tla_forall_not_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| not(a_to_p(a))) == not(tla_exists(a_to_p)),
{ unimplemented!() }

#[verifier::external_body]
proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{ unimplemented!() }

// The proven theorem
proof fn tla_forall_implies_equality1<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).implies(q)) == tla_exists(a_to_p).implies(q),
{
    let a_to_not_p = |a: A| not(a_to_p(a));
    a_to_temp_pred_equality::<T, A>(|a: A| a_to_p(a).implies(q), |a: A| a_to_not_p(a).or(q));
    temp_pred_equality::<T>(tla_forall(|a: A| a_to_p(a).implies(q)), tla_forall(|a: A| a_to_not_p(a).or(q)));
    tla_forall_or_equality::<T, A>(a_to_not_p, q);
    tla_forall_not_equality::<T, A>(a_to_p);
    temp_pred_equality::<T>(not(tla_exists(a_to_p)).or(q), tla_exists(a_to_p).implies(q));
}


// === LOGICAL TESTS ===

// LT1: Try to derive that any two arbitrary TempPreds are equal.
// If this passes, the axioms are contradictory (unsound).
// SHOULD FAIL
proof fn lt1_arbitrary_preds_not_equal()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) < 0);
    assert(p == q); // SHOULD FAIL: unrelated predicates should not be equal
}

// LT2: Try to derive false from the axioms.
// If the axioms are consistent, we should not be able to prove false.
// SHOULD FAIL
proof fn lt2_cannot_derive_false()
{
    assert(false); // SHOULD FAIL: axioms should not be contradictory
}

// LT3: valid(p) should NOT imply valid(not(p)).
// Test: if p is always true, not(p) should not be valid.
// SHOULD FAIL
proof fn lt3_valid_p_does_not_imply_valid_not_p()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    // p is valid
    assert(valid(p));
    // but not(p) should NOT be valid
    assert(valid(not(p))); // SHOULD FAIL: not(true) is false everywhere
}

// LT4: entails is NOT symmetric in general.
// p.entails(q) should NOT imply q.entails(p).
// SHOULD FAIL
proof fn lt4_entails_not_symmetric()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 5);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    // p entails q (x > 5 ==> x > 0), but q does NOT entail p (x > 0 =/=> x > 5)
    // Try to assert the reverse entailment
    assert(q.entails(p)); // SHOULD FAIL: x > 0 does not imply x > 5
}

// LT5: The theorem should not generalize to a stronger version
// where the quantifier type is restricted.
// Specifically: tla_forall over booleans should NOT equal tla_exists 
// over all A for the same predicate family.
// SHOULD FAIL
proof fn lt5_quantifier_type_mismatch()
{
    let p_bool = |b: bool| TempPred::<int>::new(
        |ex: Execution<int>| if b { (ex.nat_to_state)(0) > 0 } else { (ex.nat_to_state)(0) <= 0 }
    );
    let p_int = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );

    // These are over different domains, should not be equated
    assert(tla_forall(p_bool) == tla_forall(p_int)); // SHOULD FAIL: different predicate families
}

// LT6: Converse of the main theorem does NOT hold for different q.
// The theorem gives us an equality for specific a_to_p and q.
// But using the result with a different q should not work.
// SHOULD FAIL
proof fn lt6_cannot_cross_instantiate()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    let q1 = TempPred::<int>::new(|ex: Execution<int>| true);
    let q2 = TempPred::<int>::new(|ex: Execution<int>| false);

    // Apply theorem for q1
    tla_forall_implies_equality1::<int, int>(a_to_p, q1);

    // This should NOT let us conclude anything about q2
    assert(tla_forall(|a: int| a_to_p(a).implies(q2)) == tla_exists(a_to_p).implies(q1));
    // SHOULD FAIL: mixed q1/q2 is invalid
}

// LT7: Functional extensionality should NOT be provable without the axiom.
// Two functions that are pointwise equal should not be == without a_to_temp_pred_equality.
// SHOULD FAIL
proof fn lt7_no_implicit_function_extensionality()
{
    let f = |a: int| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > a);
    let g = |a: int| TempPred::<int>::new(|ex: Execution<int>| !((ex.nat_to_state)(0) <= a));
    // f and g compute the same thing but are structurally different closures
    // Without calling a_to_temp_pred_equality, we should NOT get f == g
    assert(f == g); // SHOULD FAIL: no automatic extensionality
}

}
