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

// === BEHAVIORAL MUTATION TESTS ===
// Each test uses valid inputs (full preconditions) but asserts a mutated postcondition.
// All tests SHOULD FAIL verification.

// Mutation Test 1: Postcondition mutated from inv to init
// The spec guarantees inv at all positions, NOT init at all positions.
// SHOULD FAIL
proof fn test_ensures_init_instead_of_inv<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures init(ex.suffix(i).head()),  // MUTATED: init instead of inv
{
    init_invariant_rec::<T>(ex, init, next, inv, i);
    // We obtain inv(ex.suffix(i).head()), but need init(ex.suffix(i).head())
    // init is not preserved by transitions and not derivable from inv
}

// Mutation Test 2: Postcondition negated — direct contradiction
// SHOULD FAIL
proof fn test_ensures_negated<T>(ex: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex.head()),
        forall |idx: nat| next(#[trigger] ex.suffix(idx).head(), ex.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex.suffix(idx).head()) ==> inv(ex.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex.suffix(idx).head()) && next(ex.suffix(idx).head(), ex.suffix(idx).head_next()) ==> inv(ex.suffix(idx).head_next()),
    ensures !inv(ex.suffix(i).head()),  // MUTATED: negated postcondition
{
    init_invariant_rec::<T>(ex, init, next, inv, i);
    // We obtain inv(ex.suffix(i).head()) = true, but ensures requires false
}

// Mutation Test 3: Postcondition applied to a different, unrelated execution
// SHOULD FAIL
proof fn test_ensures_wrong_execution<T>(ex1: Execution<T>, ex2: Execution<T>, init: StatePred<T>, next: ActionPred<T>, inv: StatePred<T>, i: nat)
    requires
        init(ex1.head()),
        forall |idx: nat| next(#[trigger] ex1.suffix(idx).head(), ex1.suffix(idx).head_next()),
        forall |idx: nat| init(#[trigger] ex1.suffix(idx).head()) ==> inv(ex1.suffix(idx).head()),
        forall |idx: nat| inv(#[trigger] ex1.suffix(idx).head()) && next(ex1.suffix(idx).head(), ex1.suffix(idx).head_next()) ==> inv(ex1.suffix(idx).head_next()),
    ensures inv(ex2.suffix(i).head()),  // MUTATED: ex2 instead of ex1
{
    init_invariant_rec::<T>(ex1, init, next, inv, i);
    // We obtain inv(ex1.suffix(i).head()), but need inv(ex2.suffix(i).head())
    // ex2 has no preconditions; inv is not guaranteed for it
}

}
