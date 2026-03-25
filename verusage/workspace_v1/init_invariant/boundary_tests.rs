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

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex), p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn init_invariant_rec<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures inv(ex.suffix(i).head()),
    decreases i,
{ unimplemented!() }

pub proof fn init_invariant<T>(spec: TempPred<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>)
    requires
        forall |s: T| #[trigger] init(s) ==> inv(s),
        forall |s, s_prime: T| inv(s) && #[trigger] next(s, s_prime) ==> inv(s_prime),
        spec.entails(lift_state(init)),
        spec.entails(always(lift_action(next))),
    ensures spec.entails(always(lift_state(inv))),
{
    assert forall |ex: Execution<T>| spec.satisfied_by(ex)
    implies #[trigger] always(lift_state(inv)).satisfied_by(ex) by {
        implies_apply(ex, spec, lift_state(init));
        implies_apply(ex, spec, always(lift_action(next)));
        always_unfold::<T>(ex, lift_action(next));
        assert forall |i: nat| inv(#[trigger] ex.suffix(i).head()) by {
            init_invariant_rec(ex, init, next, inv, i);
        };
    };
}

// ============================================================
// BOUNDARY TESTS: Violate preconditions of init_invariant
// Each test targets a different `requires` clause.
// ============================================================

// SHOULD FAIL
// Violates requires #1: forall |s| init(s) ==> inv(s)
// Counterexample: s = 0, init(0) = true, inv(0) = false
proof fn boundary_init_not_implies_inv()
{
    let init: StatePred<int> = |s: int| s == 0;
    let inv: StatePred<int> = |s: int| s > 0;
    assert(forall |s: int| #[trigger] init(s) ==> inv(s));
}

// SHOULD FAIL
// Violates requires #2: forall |s, s'| inv(s) && next(s, s') ==> inv(s')
// Counterexample: s = 4, s' = 5, inv(4) = true, next(4,5) = true, inv(5) = false
proof fn boundary_inv_not_inductive()
{
    let inv: StatePred<int> = |s: int| s >= 0 && s < 5;
    let next: ActionPred<int> = |s: int, s_prime: int| s_prime == s + 1;
    assert(forall |s: int, s_prime: int| inv(s) && #[trigger] next(s, s_prime) ==> inv(s_prime));
}

// SHOULD FAIL
// Violates requires #3: spec.entails(lift_state(init))
// A trivial spec (true for all executions) cannot entail a non-trivial init
proof fn boundary_spec_too_permissive()
{
    let init: StatePred<int> = |s: int| s == 0;
    // Asserting init holds on ALL executions is clearly false
    assert(forall |ex: Execution<int>| init(ex.head()));
}

// SHOULD FAIL
// Violates requires #4: spec.entails(always(lift_action(next)))
// An execution satisfying init at head does not necessarily satisfy next
proof fn boundary_action_not_enforced()
{
    let next: ActionPred<int> = |s: int, s_prime: int| s_prime == s + 1;
    // Execution: constant 0 at every step (violates next since 0 != 0 + 1)
    let ex = Execution::<int> { nat_to_state: |i: nat| 0 };
    assert(lift_action(next).satisfied_by(ex));
}

}
