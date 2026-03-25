use vstd::prelude::*;

fn main() {}

verus! {

// === Definitions from source ===

pub type ActionPred<T> = spec_fn(T, T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }

    pub open spec fn head_next(self) -> T {
        (self.nat_to_state)(1)
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

pub open spec fn lift_action<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| action_pred(ex.head(), ex.head_next()))
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

proof fn always_lift_action_unfold<T>(ex: Execution<T>, p: ActionPred<T>)
    requires always(lift_action(p)).satisfied_by(ex),
    ensures forall |i| p(#[trigger] ex.suffix(i).head(), ex.suffix(i).head_next()),
{
    always_unfold::<T>(ex, lift_action(p));
}

// === Logical Tests ===
// Properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL
// Transitivity: p(s_0, s_2) does NOT follow from forall i. p(s_i, s_{i+1}).
// The spec provides consecutive-step relations, not skip-step ones.
proof fn test_logical_transitivity_not_implied(ex: Execution<int>, p: ActionPred<int>)
    requires always(lift_action(p)).satisfied_by(ex),
{
    always_lift_action_unfold(ex, p);
    // p(s_0, s_2): skipping one intermediate state
    assert(p(ex.suffix(0).head(), ex.suffix(2).head()));
}

// SHOULD FAIL
// Cross-predicate: always(lift_action(p)) should NOT imply always(lift_action(q))
// for an unrelated predicate q.
proof fn test_logical_cross_predicate(ex: Execution<int>, p: ActionPred<int>, q: ActionPred<int>)
    requires always(lift_action(p)).satisfied_by(ex),
{
    always_lift_action_unfold(ex, p);
    // Assert q holds for the same execution — should not follow from p
    assert(q(ex.suffix(0).head(), ex.suffix(0).head_next()));
}

// SHOULD FAIL
// State equality: the spec should NOT imply consecutive states are equal.
// Knowing p(s_i, s_{i+1}) for all i says nothing about s_i == s_{i+1}.
proof fn test_logical_state_equality_not_implied(ex: Execution<int>, p: ActionPred<int>)
    requires always(lift_action(p)).satisfied_by(ex),
{
    always_lift_action_unfold(ex, p);
    assert(ex.suffix(0).head() == ex.suffix(0).head_next());
}

}
