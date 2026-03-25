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

// ========== Behavioral Mutation Tests ==========

// SHOULD FAIL
// Test 1: Mutate the execution — always(p) on ex should NOT imply always(p) on unrelated ex2.
proof fn test_mutation_wrong_execution(ex: Execution<int>, ex2: Execution<int>, p: TempPred<int>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex2.suffix(i)),
{
    always_propagate_forwards::<int>(ex, p, i);
}

// SHOULD FAIL
// Test 2: Mutate the predicate — always(p) on ex should NOT imply always(q) on ex.suffix(i).
proof fn test_mutation_wrong_predicate(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(q).satisfied_by(ex.suffix(i)),
{
    always_propagate_forwards::<int>(ex, p, i);
}

// SHOULD FAIL
// Test 3: Negate the correct conclusion — always(p) on ex should NOT imply NOT always(p) on suffix.
proof fn test_mutation_negated_conclusion(ex: Execution<int>, p: TempPred<int>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures !always(p).satisfied_by(ex.suffix(i)),
{
    always_propagate_forwards::<int>(ex, p, i);
}

// SHOULD FAIL
// Test 4: Mutate the suffix index — always(p).satisfied_by(ex.suffix(i)) should NOT imply
// always(p).satisfied_by(ex.suffix(j)) for arbitrary j != i without the original precondition.
proof fn test_mutation_wrong_suffix_index(ex: Execution<int>, p: TempPred<int>, i: nat, j: nat)
    requires always(p).satisfied_by(ex.suffix(i)),
    ensures always(p).satisfied_by(ex.suffix(j)),
{
}

}
