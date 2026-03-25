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

// ========== Logical Tests ==========

// SHOULD FAIL
// Test 1: Converse — always(p) on a suffix does NOT imply always(p) on the original execution.
// The forward direction (always_propagate_forwards) is proven, but backward propagation
// would require knowledge about states before position i, which is lost.
proof fn test_logical_converse_backward_propagation(ex: Execution<int>, p: TempPred<int>, i: nat)
    requires always(p).satisfied_by(ex.suffix(i)),
    ensures always(p).satisfied_by(ex),
{
}

// SHOULD FAIL
// Test 2: Single point does NOT imply always — p holding at one execution
// does not mean p holds at ALL suffixes.
proof fn test_logical_single_point_to_always(ex: Execution<int>, p: TempPred<int>)
    requires p.satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
}

// SHOULD FAIL
// Test 3: Determinism — two executions both satisfying always(p) need NOT be equal.
// Different executions can satisfy the same temporal property.
proof fn test_logical_determinism(ex1: Execution<int>, ex2: Execution<int>, p: TempPred<int>)
    requires
        always(p).satisfied_by(ex1),
        always(p).satisfied_by(ex2),
    ensures ex1 == ex2,
{
}

// SHOULD FAIL
// Test 4: Predicate conflation — two predicates that are both always-satisfied by the same
// execution need NOT be the same predicate.
proof fn test_logical_predicate_conflation(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        always(p).satisfied_by(ex),
        always(q).satisfied_by(ex),
    ensures p == q,
{
}

}
