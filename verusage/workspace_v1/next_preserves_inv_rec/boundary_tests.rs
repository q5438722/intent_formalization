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

// === Boundary Tests ===
// These tests violate preconditions and should be REJECTED by the verifier.

// SHOULD FAIL: Missing initial invariant condition
// The spec requires inv.satisfied_by(ex) as a base case for induction.
// Without it, the induction has no anchor.
proof fn test_boundary_missing_initial_inv<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        // inv.satisfied_by(ex), // OMITTED
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)),
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Missing next-step universal condition
// The spec requires next to hold at every position.
// Without it, the preservation step has an unsatisfied antecedent.
proof fn test_boundary_missing_next<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        // forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)), // OMITTED
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)),
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: Missing preservation (inductive step) condition
// The spec requires that inv is preserved from step idx to idx+1 when next holds.
// Without it, the induction cannot proceed past the base case.
proof fn test_boundary_missing_preservation<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        // preservation step OMITTED
    ensures inv.satisfied_by(ex.suffix(i)),
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: No preconditions at all
// With no assumptions, the conclusion cannot possibly be established.
proof fn test_boundary_no_preconditions<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    ensures inv.satisfied_by(ex.suffix(i)),
{
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

}
