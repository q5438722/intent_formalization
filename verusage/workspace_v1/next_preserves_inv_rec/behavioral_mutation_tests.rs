use vstd::prelude::*;

fn main() {}

verus! {

// === Definitions from source ===

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
    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }
}

proof fn next_preserves_inv_rec<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)),
    decreases i,
{
    if i == 0 {
        execution_equality::<T>(ex, ex.suffix(0));
    } else {
        next_preserves_inv_rec::<T>(ex, next, inv, (i-1) as nat);
    }
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// === Behavioral Mutation Tests ===
// These tests start from valid inputs but mutate the expected output/relation.

// SHOULD FAIL: Negated conclusion
// The spec ensures inv.satisfied_by(ex.suffix(i)), not its negation.
// After calling the function we know inv holds, so the negation is unprovable.
proof fn test_mutation_negated_conclusion<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures !inv.satisfied_by(ex.suffix(i)), // MUTATED: negated
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Wrong execution in conclusion
// The spec only guarantees inv on the GIVEN execution ex, not on an arbitrary ex2.
proof fn test_mutation_wrong_execution<T>(ex: Execution<T>, ex2: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex2.suffix(i)), // MUTATED: ex2 instead of ex
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Contradictory conjunction in conclusion
// The spec ensures inv holds at suffix(i). This test additionally claims next does NOT hold,
// which contradicts the universal next precondition.
proof fn test_mutation_contradictory_conjunction<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)) && !next.satisfied_by(ex.suffix(i)), // MUTATED: added false conjunct
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Inverted requires and ensures
// If inv does NOT hold initially, claim it holds at suffix(i).
// This reverses the logical direction of the specification.
proof fn test_mutation_inverted_precondition<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        !inv.satisfied_by(ex), // MUTATED: negated initial condition
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| !inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> !inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)), // claim positive from negated chain
{
}

}
