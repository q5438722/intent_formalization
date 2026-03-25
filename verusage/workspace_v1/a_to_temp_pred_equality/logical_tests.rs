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

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
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

proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{
    assert forall |a: A| #[trigger] p(a) == q(a) by {
        temp_pred_equality::<T>(p(a), q(a));
    };
    assert(p =~= q);
}

// === LOGICAL TESTS ===

// Test 1: One-directional entailment does NOT imply equality.
// Only p.entails(q) is given, the reverse is not. Equality should not follow.
// SHOULD FAIL
proof fn test_logical_one_direction_implies_equality(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
{
    assert(p == q);
}

// Test 2: Soundness check — calling the axiom with valid preconditions should not
// allow deriving false. If this passes, the axiom is unsound.
// SHOULD FAIL
proof fn test_logical_soundness(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    temp_pred_equality::<int>(p, q);
    assert(false);
}

// Test 3: Mutual entailment should NOT yield equality without calling the lemma.
// Verus cannot derive extensional equality for TempPred without the external axiom.
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
