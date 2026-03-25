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
// BEHAVIORAL MUTATION TESTS
// Start from valid inputs, mutate expected outputs or relations.
// Each test checks that an incorrect mutation is rejected.
// ============================================================

// SHOULD FAIL
// Mutation: wrong invariant constant (s == 1 instead of correct s == 0)
// On an execution always at 0, inv(0) = (0 == 1) = false
proof fn mutation_wrong_inv_constant()
{
    let wrong_inv: StatePred<int> = |s: int| s == 1;
    let ex = Execution::<int> { nat_to_state: |i: nat| 0 };
    // head = 0, wrong_inv(0) = false
    assert(lift_state(wrong_inv).satisfied_by(ex));
}

// SHOULD FAIL
// Mutation: off-by-one in invariant (s > 0 instead of correct s >= 0)
// On an execution starting at 0, inv(0) = (0 > 0) = false
proof fn mutation_off_by_one_inv()
{
    let wrong_inv: StatePred<int> = |s: int| s > 0;
    let ex = Execution::<int> { nat_to_state: |i: nat| 0 };
    // head = 0, wrong_inv(0) = false
    assert(lift_state(wrong_inv).satisfied_by(ex));
}

// SHOULD FAIL
// Mutation: wrong action relation (s' == s + 1 instead of correct s' == s)
// On a constant-0 execution, next(0, 0) with wrong_next requires 0 == 0+1 = false
proof fn mutation_wrong_action_relation()
{
    let wrong_next: ActionPred<int> = |s: int, s_prime: int| s_prime == s + 1;
    let ex = Execution::<int> { nat_to_state: |i: nat| 0 };
    // head = 0, head_next = 0, wrong_next(0, 0) = (0 == 1) = false
    assert(lift_action(wrong_next).satisfied_by(ex));
}

}
