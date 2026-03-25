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
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    unimplemented!()
}

// === LOGICAL TESTS ===

// Test 1: Entailment is NOT symmetric.
// p.entails(q) does NOT imply q.entails(p).
// SHOULD FAIL
proof fn test_logical_entails_not_symmetric(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
{
    assert(q.entails(p));
}

// Test 2: Existential satisfaction at one execution does NOT imply universal validity.
// tla_exists(a_to_p).satisfied_by(ex) is for one ex; valid is for all ex.
// SHOULD FAIL
proof fn test_logical_single_satisfied_not_valid(ex: Execution<int>, a_to_p: spec_fn(int) -> TempPred<int>)
    requires tla_exists(a_to_p).satisfied_by(ex),
{
    assert(valid(tla_exists(a_to_p)));
}

// Test 3: Mutual entailment should NOT yield equality without calling the axiom lemma.
// Verus cannot derive p == q from extensional equivalence alone without the external axiom.
// SHOULD FAIL
proof fn test_logical_equality_without_lemma(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    // Deliberately do NOT call temp_pred_equality
    assert(p == q);
}

}
