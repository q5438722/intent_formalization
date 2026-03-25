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

// === BOUNDARY TESTS ===

// Test 1: Call always_lift_state_unfold without any precondition
// SHOULD FAIL: precondition always(lift_state(p)).satisfied_by(ex) is not established
proof fn test_boundary_no_precondition(ex: Execution<int>, p: StatePred<int>)
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

// Test 2: Only the head of the execution satisfies p, not all suffixes
// SHOULD FAIL: p(ex.head()) does not imply always(lift_state(p)).satisfied_by(ex)
proof fn test_boundary_only_head(ex: Execution<int>, p: StatePred<int>)
    requires p(ex.head()),
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

// Test 3: p holds at a finite prefix (positions 0, 1, 2) but not guaranteed for all
// SHOULD FAIL: finite prefix does not establish always
proof fn test_boundary_finite_prefix(ex: Execution<int>, p: StatePred<int>)
    requires
        p(ex.suffix(0 as nat).head()),
        p(ex.suffix(1 as nat).head()),
        p(ex.suffix(2 as nat).head()),
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

}
