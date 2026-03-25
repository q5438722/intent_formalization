use vstd::prelude::*;

fn main() {}

verus!{

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
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
}

proof fn init_invariant_rec<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures inv(ex.suffix(i).head()),
    decreases i,
{
    if i == 0 {
        assert(init(ex.suffix(0).head()));
    } else {
        init_invariant_rec::<T>(ex, init, next, inv, (i-1) as nat);
    }
}

// === LOGICAL TESTS ===
// Each test asserts a property NOT explicitly guaranteed by the specification.
// All tests SHOULD FAIL verification.

// Logical Test 1: init holds at all execution positions (not just head)
// The spec only requires init(ex.head()), not init at arbitrary positions.
// SHOULD FAIL
proof fn test_init_propagates_everywhere<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures init(ex.suffix(i).head()),  // NOT GUARANTEED: init is only at head
{
    init_invariant_rec::<T>(ex, init, next, inv, i);
    // We obtain inv(ex.suffix(i).head()), but init is a weaker initial condition,
    // not an invariant property. init ==> inv, not inv ==> init.
}

// Logical Test 2: inv holds for ANY state, not just execution states
// The spec proves inv along the execution trace, not universally.
// SHOULD FAIL
proof fn test_inv_holds_for_arbitrary_state<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, s: T)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures inv(s),  // NOT GUARANTEED: s is arbitrary, not on the execution
{
    // s is not necessarily on the execution trace
    // inv is only proven for execution states, not all states of type T
}

// Logical Test 3: Two executions with the same predicates must have equal states
// The spec does not imply determinism of executions.
// SHOULD FAIL
proof fn test_execution_uniqueness<T>(ex1: Execution<T>, ex2: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex1.head()),
        init(ex2.head()),
        forall |idx: nat| next(#[trigger] ex1.suffix(idx).head(), ex1.suffix(idx).head_next()),
        forall |idx: nat| next(#[trigger] ex2.suffix(idx).head(), ex2.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex1.suffix(idx).head()) ==> inv(ex1.suffix(idx).head()),
        forall |idx: nat| init(#[trigger] ex2.suffix(idx).head()) ==> inv(ex2.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex1.suffix(idx).head()) && next(ex1.suffix(idx).head(), ex1.suffix(idx).head_next()) ==> inv(ex1.suffix(idx).head_next()),
        forall |idx: nat| inv(#[trigger] ex2.suffix(idx).head()) && next(ex2.suffix(idx).head(), ex2.suffix(idx).head_next()) ==> inv(ex2.suffix(idx).head_next()),
    ensures ex1.suffix(i).head() == ex2.suffix(i).head(),  // NOT GUARANTEED: different executions can diverge
{
    init_invariant_rec::<T>(ex1, init, next, inv, i);
    init_invariant_rec::<T>(ex2, init, next, inv, i);
    // Both inv(ex1...) and inv(ex2...) hold, but states can still differ
}

// Logical Test 4: inv && next implies init at the next state (confused direction)
// The spec says init ==> inv, NOT inv ==> init. Transitions preserve inv, not init.
// SHOULD FAIL
proof fn test_inv_next_implies_init<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> init(ex.suffix(idx).head_next()),
    // NOT GUARANTEED: the spec says inv is preserved, not that init is re-established
{
    // The implication init ==> inv does not reverse to inv ==> init
    // Transitions preserve inv, they do not produce init
}

}
