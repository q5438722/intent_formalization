use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions from target ===
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
        TempPred {
            pred: pred,
        }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

#[verifier::external_body]
proof fn eventually_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures exists |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

proof fn eventually_propagate_backwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures eventually(p).satisfied_by(ex),
{
    eventually_unfold::<T>(ex.suffix(i), p);
    let witness_idx = eventually_choose_witness(ex.suffix(i), p);
    execution_equality::<T>(ex.suffix(i).suffix(witness_idx), ex.suffix(i + witness_idx));
}

spec fn eventually_choose_witness<T>(ex: Execution<T>, p: TempPred<T>) -> nat
    recommends exists |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    let witness = choose |i| p.satisfied_by(#[trigger] ex.suffix(i));
    witness
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// === Boundary Tests ===
// These tests violate preconditions to check if invalid inputs are rejected.

// SHOULD FAIL: Call eventually_unfold without its precondition.
// If this passes, the precondition on eventually_unfold is vacuous.
proof fn boundary_test_unfold_missing_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures exists |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Call eventually_propagate_backwards without its precondition.
// Attempts to derive eventually(p) from nothing.
proof fn boundary_test_propagate_missing_precondition<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    ensures eventually(p).satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Call execution_equality without pointwise equality.
// Two arbitrary executions should not be provably equal.
proof fn boundary_test_equality_missing_precondition<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality::<T>(ex1, ex2);
}

// SHOULD FAIL: Call eventually_propagate_backwards with a different predicate.
// Uses q's eventually as precondition but tries to conclude p's eventually.
proof fn boundary_test_wrong_predicate<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, i: nat)
    requires eventually(q).satisfied_by(ex.suffix(i)),
    ensures eventually(p).satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Call eventually_propagate_backwards with a different execution.
// Precondition on ex2 but conclusion about ex1.
proof fn boundary_test_wrong_execution<T>(ex1: Execution<T>, ex2: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex2.suffix(i)),
    ensures eventually(p).satisfied_by(ex1),
{
    eventually_propagate_backwards::<T>(ex1, p, i);
}

}
