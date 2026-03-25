use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (from target) =====

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
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

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

proof fn always_double<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures always(always(p)).satisfied_by(ex),
{
    always_unfold::<T>(ex, p);
    assert forall |i| always(p).satisfied_by(#[trigger] ex.suffix(i)) by {
        always_propagate_forwards::<T>(ex, p, i);
    };
}

// ===== Boundary Tests =====

// Test 1: Call always_unfold without its precondition (no requires)
// SHOULD FAIL
proof fn boundary_test_unfold_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// Test 2: Call always_propagate_forwards without its precondition
// SHOULD FAIL
proof fn boundary_test_propagate_no_precondition<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    always_propagate_forwards::<T>(ex, p, i);
}

// Test 3: Call always_double without its precondition
// SHOULD FAIL
proof fn boundary_test_double_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures always(always(p)).satisfied_by(ex),
{
    always_double::<T>(ex, p);
}

}
