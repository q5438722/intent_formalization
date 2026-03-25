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

// === Logical Tests ===
// These test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: Unique witness — eventually does not guarantee uniqueness.
// The existential in eventually(p) may have multiple witnesses.
proof fn logical_test_unique_witness<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures
        forall |i: nat, j: nat|
            (p.satisfied_by(#[trigger] ex.suffix(i)) && p.satisfied_by(#[trigger] ex.suffix(j)))
            ==> i == j,
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Eventually implies immediately — p does not necessarily hold at ex.
// This is a logical strengthening: eventually(p) does NOT entail p.
proof fn logical_test_eventually_implies_now<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Universal eventually — not every predicate eventually holds on every execution.
// This tests whether the spec accidentally admits trivial satisfiability.
proof fn logical_test_universal_eventually<T>(ex: Execution<T>, p: TempPred<T>)
    ensures eventually(p).satisfied_by(ex),
{
    // No precondition — cannot conjure eventually from nothing
}

// SHOULD FAIL: Forward propagation — eventually(p) at ex does NOT imply eventually(p) at suffix(i).
// The spec only supports backward propagation. Forward requires the witness to be >= i.
proof fn logical_test_forward_propagation<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex),
    ensures eventually(p).satisfied_by(ex.suffix(i)),
{
    eventually_unfold::<T>(ex, p);
    // Cannot prove: the witness j might be < i, so ex.suffix(i) might miss it
}

// SHOULD FAIL: Exploit execution_equality to prove false equality.
// Two distinct constant executions should not be equal via the axiom.
proof fn logical_test_false_equality_via_axiom()
    ensures
        (Execution::<int> { nat_to_state: |i: nat| 0int }) ==
        (Execution::<int> { nat_to_state: |i: nat| 1int }),
{
    let ex0 = Execution::<int> { nat_to_state: |i: nat| 0int };
    let ex1 = Execution::<int> { nat_to_state: |i: nat| 1int };
    execution_equality::<int>(ex0, ex1);
}

}
