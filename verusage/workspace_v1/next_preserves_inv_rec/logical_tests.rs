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

// === Logical Tests ===
// These test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: Converse direction — backward induction is not supported
// Knowing inv at suffix(i) does NOT allow concluding inv at the original execution.
// The induction only goes forward (0 → 1 → 2 → ...), not backward.
proof fn test_logical_converse<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        // NO initial inv.satisfied_by(ex)
        inv.satisfied_by(ex.suffix(i)),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex), // SHOULD FAIL: can't go backward
{
}

// SHOULD FAIL: Arbitrary predicate generalization
// The spec only guarantees that INV holds at suffix(i).
// An unrelated predicate p should NOT be provable from the same assumptions.
proof fn test_logical_arbitrary_predicate<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, p: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures p.satisfied_by(ex.suffix(i)), // SHOULD FAIL: p is unrelated to inv
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Determinism — two executions satisfying the same inv/next have the same states
// The spec says nothing about state equality; it only tracks predicate satisfaction.
proof fn test_logical_determinism<T>(ex1: Execution<T>, ex2: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex1),
        inv.satisfied_by(ex2),
        forall |idx| next.satisfied_by(#[trigger] ex1.suffix(idx)),
        forall |idx| next.satisfied_by(#[trigger] ex2.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex1.suffix(idx)) && next.satisfied_by(ex1.suffix(idx))
            ==> inv.satisfied_by(ex1.suffix(idx + 1)),
        forall |idx| inv.satisfied_by(#[trigger] ex2.suffix(idx)) && next.satisfied_by(ex2.suffix(idx))
            ==> inv.satisfied_by(ex2.suffix(idx + 1)),
    ensures (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i), // SHOULD FAIL: states not constrained
{
    next_preserves_inv_rec::<T>(ex1, next, inv, i);
    next_preserves_inv_rec::<T>(ex2, next, inv, i);
}

// SHOULD FAIL: Soundness — the spec should not entail false
// If the preconditions are satisfiable (which they are), false should not be provable.
proof fn test_logical_soundness<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures false, // SHOULD FAIL: can't prove false from consistent assumptions
{
    next_preserves_inv_rec::<T>(ex, next, inv, 0);
}

}
