use vstd::prelude::*;

fn main() {}

verus!{

// === Type definitions (from init_invariant.rs) ===

pub type StatePred<T> = spec_fn(T) -> bool;
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
        Execution { nat_to_state: |i: nat| (self.nat_to_state)(i + pos) }
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
    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn lift_action<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| action_pred(ex.head(), ex.head_next()))
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ============================================================
// LOGICAL TESTS
// Test properties NOT explicitly guaranteed by the specification.
// These probe unintended reasoning / structural assumptions.
// ============================================================

// SHOULD FAIL
// Reverse of precondition #1: inv(s) ==> init(s) is NOT guaranteed.
// init is stricter (s == 0) than inv (s >= 0), so inv does not imply init.
// Counterexample: s = 5, inv(5) = true, init(5) = false
proof fn logical_inv_does_not_imply_init()
{
    let init: StatePred<int> = |s: int| s == 0;
    let inv: StatePred<int> = |s: int| s >= 0;
    assert(forall |s: int| #[trigger] inv(s) ==> init(s));
}

// SHOULD FAIL
// Determinism: next does NOT guarantee a unique successor.
// With non-deterministic next (s' >= s), two different successors are possible.
// Counterexample: s=0, s1=1, s2=2, both satisfy next(0,1) and next(0,2)
proof fn logical_next_not_deterministic()
{
    let next: ActionPred<int> = |s: int, s_prime: int| s_prime >= s;
    assert(forall |s: int, s1: int, s2: int|
        #[trigger] next(s, s1) && #[trigger] next(s, s2) ==> s1 == s2
    );
}

// SHOULD FAIL
// Reverse entailment: always(inv) does NOT imply init at head.
// An execution can always satisfy inv (s >= 0) without starting at init (s == 0).
// Concrete: execution at constant 5 satisfies always(s >= 0) but not init(s == 0).
proof fn logical_always_inv_not_implies_init()
{
    let init: StatePred<int> = |s: int| s == 0;
    let inv: StatePred<int> = |s: int| s >= 0;
    let ex = Execution::<int> { nat_to_state: |i: nat| 5 };
    // always(inv) holds: every state is 5 >= 0
    assume(always(lift_state(inv)).satisfied_by(ex));
    // But init does NOT hold: head is 5 != 0
    assert(lift_state(init).satisfied_by(ex));
}

}
