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

// === Boundary Tests ===
// These tests violate preconditions and should be rejected by Verus.

// SHOULD FAIL
// Constant execution where action predicate is never satisfied.
// ex = (0, 0, 0, ...), p(a,b) = (b > a). p(0,0) = false.
proof fn test_boundary_constant_execution_strict_action() {
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p: ActionPred<int> = |a: int, b: int| b > a;
    always_lift_action_unfold(ex, p);
}

// SHOULD FAIL
// Always-false action predicate: p = |_,_| false. Precondition can never hold.
proof fn test_boundary_always_false_action() {
    let ex = Execution::<int> { nat_to_state: |i: nat| i as int };
    let p: ActionPred<int> = |a: int, b: int| false;
    always_lift_action_unfold(ex, p);
}

// SHOULD FAIL
// Arbitrary execution and predicate with no precondition proof.
// Without any assumption about always(lift_action(p)), the call should fail.
proof fn test_boundary_no_precondition(ex: Execution<int>, p: ActionPred<int>) {
    always_lift_action_unfold(ex, p);
}

}
