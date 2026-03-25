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

// === Behavioral Mutation Tests ===
// These tests start from valid inputs but mutate expected outputs or relations.

// SHOULD FAIL: Strengthen conclusion from eventually(p) to p directly.
// Removes the "eventually" wrapper — p should not hold immediately at ex.
proof fn behavioral_test_drop_eventually<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex),
{
    eventually_propagate_backwards::<T>(ex, p, i);
}

// SHOULD FAIL: Mutate conclusion to claim p holds at suffix(i) directly.
// eventually(p) at suffix(i) does NOT mean p holds at suffix(i) itself.
proof fn behavioral_test_p_at_suffix<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex.suffix(i)),
{
    eventually_unfold::<T>(ex.suffix(i), p);
}

// SHOULD FAIL: Claim that eventually(p) at ex implies p at suffix(0).
// "Eventually" does not mean "now" — there is no bound on when p occurs.
proof fn behavioral_test_eventually_means_now<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(0)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Mutate to claim a bounded witness exists.
// eventually(p) provides an existential witness but not one bounded by 1.
proof fn behavioral_test_bounded_witness<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(0)) || p.satisfied_by(ex.suffix(1)),
{
    eventually_unfold::<T>(ex, p);
}

// SHOULD FAIL: Mutate to propagate p (not eventually(p)) backwards through suffix.
// The lemma propagates eventually(p) backwards, not p itself.
proof fn behavioral_test_propagate_p_not_eventually<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires p.satisfied_by(ex.suffix(i)),
    ensures p.satisfied_by(ex),
{
    // p at suffix(i) does NOT mean p at ex
}

}
