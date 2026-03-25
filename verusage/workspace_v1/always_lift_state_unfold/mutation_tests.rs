use vstd::prelude::*;

fn main() {}

verus! {

pub type StatePred<T> = spec_fn(T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }

    pub open spec fn suffix(self, pos: nat) -> Self {
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
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
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

proof fn always_lift_state_unfold<T>(ex: Execution<T>, p: StatePred<T>)
    requires always(lift_state(p)).satisfied_by(ex),
    ensures forall |i: nat| p(#[trigger] ex.suffix(i).head()),
{
    always_unfold::<T>(ex, lift_state(p));
}

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: Negate the postcondition at position 0
// SHOULD FAIL: postcondition guarantees p(ex.suffix(0).head()), asserting its negation contradicts it
proof fn test_mutation_negate_at_zero(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(!p(ex.suffix(0 as nat).head())); // SHOULD FAIL
}

// Test 2: Assert postcondition for a different arbitrary predicate q
// SHOULD FAIL: the lemma only guarantees results for p, not for an unrelated q
proof fn test_mutation_wrong_predicate(ex: Execution<int>, p: StatePred<int>, q: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat| q(#[trigger] ex.suffix(i).head())); // SHOULD FAIL
}

// Test 3: Assert postcondition for a different execution
// SHOULD FAIL: the lemma only guarantees results for the given execution ex1, not ex2
proof fn test_mutation_wrong_execution(ex1: Execution<int>, ex2: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex1),
{
    always_lift_state_unfold::<int>(ex1, p);
    assert(forall |i: nat| p(#[trigger] ex2.suffix(i).head())); // SHOULD FAIL
}

}
