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

// === LOGICAL TESTS ===

// Test 1: Determinism — two executions satisfying the same always(lift_state(p))
// must have the same head state
// SHOULD FAIL: the spec does not guarantee executions are unique or deterministic
proof fn test_logical_determinism(ex1: Execution<int>, ex2: Execution<int>, p: StatePred<int>)
    requires
        always(lift_state(p)).satisfied_by(ex1),
        always(lift_state(p)).satisfied_by(ex2),
{
    always_lift_state_unfold::<int>(ex1, p);
    always_lift_state_unfold::<int>(ex2, p);
    assert(ex1.head() == ex2.head()); // SHOULD FAIL
}

// Test 2: Structural collapse — always(lift_state(p)) implies all states are equal
// SHOULD FAIL: p can hold on many different state values
proof fn test_logical_all_states_equal(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat, j: nat|
        #[trigger] (ex.nat_to_state)(i) == #[trigger] (ex.nat_to_state)(j)); // SHOULD FAIL
}

// Test 3: Stronger unrelated predicate — always(lift_state(p)) implies
// an arbitrary stronger property (all states are positive)
// SHOULD FAIL: the spec makes no claim about state values beyond p
proof fn test_logical_stronger_property(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat| (#[trigger] ex.suffix(i).head()) > 0); // SHOULD FAIL
}

}
