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

proof fn tla_forall_not_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| not(a_to_p(a))) == not(tla_exists(a_to_p)),
{
    let a_to_not_p = |a: A| not(a_to_p(a));
    assert forall |ex| #[trigger] tla_forall(a_to_not_p).satisfied_by(ex)
    implies not(tla_exists(a_to_p)).satisfied_by(ex) by {
        assert forall |a| !#[trigger] a_to_p(a).satisfied_by(ex) by {
            tla_forall_unfold::<T, A>(ex, a_to_not_p);
            assert(a_to_not_p(a).satisfied_by(ex));
        };
    };

    temp_pred_equality::<T>(tla_forall(|a: A| not(a_to_p(a))), not(tla_exists(a_to_p)));
}

// === LOGICAL TESTS ===

// Test 1: Assert the De Morgan equality WITHOUT calling the lemma.
// The equality should not be derivable without the proof; Verus can't infer
// extensional equality for TempPred on its own.
// SHOULD FAIL
proof fn test_logical_equality_without_lemma(a_to_p: spec_fn(int) -> TempPred<int>)
{
    // Deliberately do NOT call tla_forall_not_equality
    assert(tla_forall(|a: int| not(a_to_p(a))) == not(tla_exists(a_to_p)));
}

// Test 2: Soundness check — using the lemma should not allow deriving false.
// If this passes, the axioms are unsound.
// SHOULD FAIL
proof fn test_logical_soundness(a_to_p: spec_fn(int) -> TempPred<int>)
{
    tla_forall_not_equality::<int, int>(a_to_p);
    assert(false);
}

// Test 3: The lemma should not imply validity of the forall predicate itself.
// Knowing ∀a.¬P(a) == ¬∃a.P(a) does NOT mean ∀a.¬P(a) is valid.
// SHOULD FAIL
proof fn test_logical_forall_not_valid(a_to_p: spec_fn(int) -> TempPred<int>)
{
    tla_forall_not_equality::<int, int>(a_to_p);
    assert(valid(tla_forall(|a: int| not(a_to_p(a)))));
}

}
