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

// =====================================================================
// BOUNDARY TESTS — Violate preconditions
// =====================================================================

// SHOULD FAIL: Call eventually_unfold without its precondition.
proof fn boundary_test_unfold_missing_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures exists |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Call eventually_propagate_backwards without its precondition.
proof fn boundary_test_propagate_missing_precondition<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    ensures eventually(p).satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Call execution_equality without pointwise equality.
proof fn boundary_test_equality_missing_precondition<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality::<T>(ex1, ex2);
}

// SHOULD FAIL: Use wrong predicate — precondition on q, conclusion about p.
proof fn boundary_test_wrong_predicate<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, i: nat)
    requires eventually(q).satisfied_by(ex.suffix(i)),
    ensures eventually(p).satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Use wrong execution — precondition on ex2, conclusion about ex1.
proof fn boundary_test_wrong_execution<T>(ex1: Execution<T>, ex2: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex2.suffix(i)),
    ensures eventually(p).satisfied_by(ex1),
{
    eventually_propagate_backwards::<T>(ex1, p, i);
}

// =====================================================================
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs
// =====================================================================

// SHOULD FAIL: Strengthen conclusion from eventually(p) to p.
proof fn behavioral_test_drop_eventually<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Mutate conclusion — p at suffix(i) directly.
proof fn behavioral_test_p_at_suffix<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex.suffix(i)),
{
    eventually_unfold::<T>(ex.suffix(i), p);
}

// SHOULD FAIL: Claim eventually(p) means p at suffix(0).
proof fn behavioral_test_eventually_means_now<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(0)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Bounded witness — p at index 0 or 1.
proof fn behavioral_test_bounded_witness<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(0)) || p.satisfied_by(ex.suffix(1)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Propagate p backwards (not eventually(p)).
proof fn behavioral_test_propagate_p_not_eventually<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires p.satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex),
{
}

// =====================================================================
// LOGICAL TESTS — Properties not explicitly guaranteed
// =====================================================================

// SHOULD FAIL: Unique witness — eventually has no uniqueness guarantee.
proof fn logical_test_unique_witness<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures
        forall |i: nat, j: nat|
            (p.satisfied_by(#[trigger] ex.suffix(i)) && p.satisfied_by(#[trigger] ex.suffix(j)))
            ==> i == j,
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: eventually(p) does NOT entail p.
proof fn logical_test_eventually_implies_now<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Universal eventually — not provable without precondition.
proof fn logical_test_universal_eventually<T>(ex: Execution<T>, p: TempPred<T>)
    ensures eventually(p).satisfied_by(ex),
{
}

// SHOULD FAIL: Forward propagation — spec only supports backward.
proof fn logical_test_forward_propagation<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex),
    ensures eventually(p).satisfied_by(ex.suffix(i)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: False equality via execution_equality — precondition blocks it.
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
