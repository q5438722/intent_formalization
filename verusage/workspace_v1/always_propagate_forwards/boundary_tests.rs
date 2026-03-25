use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions (from target) ==========

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

proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
    assert forall |j| p.satisfied_by(#[trigger] ex.suffix(i).suffix(j)) by {
        execution_equality::<T>(ex.suffix(i + j), ex.suffix(i).suffix(j));
    };
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// ========== Boundary Tests ==========

// SHOULD FAIL
// Test 1: Call always_propagate_forwards without its precondition.
// An arbitrary execution and predicate need not satisfy always(p).
proof fn test_boundary_propagate_no_precondition(ex: Execution<int>, p: TempPred<int>)
{
    always_propagate_forwards::<int>(ex, p, 1nat);
}

// SHOULD FAIL
// Test 2: Call always_unfold without its precondition.
// Cannot unfold always(p) if it does not hold.
proof fn test_boundary_unfold_no_precondition(ex: Execution<int>, p: TempPred<int>)
{
    always_unfold::<int>(ex, p);
}

// SHOULD FAIL
// Test 3: Call execution_equality on two arbitrary (potentially different) executions.
// Pointwise equality is not guaranteed for arbitrary executions.
proof fn test_boundary_exec_equality_no_precondition(ex1: Execution<int>, ex2: Execution<int>)
    ensures ex1 == ex2,
{
    execution_equality::<int>(ex1, ex2);
}

// SHOULD FAIL
// Test 4: Propagate with negated precondition — explicitly assume always(p) does NOT hold.
proof fn test_boundary_propagate_negated_precondition(ex: Execution<int>, p: TempPred<int>, i: nat)
    requires !always(p).satisfied_by(ex),
{
    always_propagate_forwards::<int>(ex, p, i);
}

}
