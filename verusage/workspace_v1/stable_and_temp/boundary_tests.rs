use vstd::prelude::*;

fn main() {}

verus!{

// ===== Type definitions (from target) =====

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
    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred { pred: pred }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn stable_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires stable(p).satisfied_by(ex),
    ensures p.satisfied_by(ex) ==> forall |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

// ===== Boundary Tests =====

// Boundary Test 1: Missing precondition valid(stable(q))
// Only one of two required stability assumptions is provided.
// The conjunction p.and(q) should NOT be provably stable from p alone.
// SHOULD FAIL
proof fn test_boundary_missing_stable_q<T>(p: TempPred<T>, q: TempPred<T>)
    requires valid(stable(p)),
    ensures valid(stable(p.and(q))),
{
}

// Boundary Test 2: Missing BOTH preconditions
// With no stability assumptions at all, stable(p.and(q)) is not derivable.
// SHOULD FAIL
proof fn test_boundary_no_preconditions<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(stable(p.and(q))),
{
}

// Boundary Test 3: Calling stable_unfold without its precondition
// stable_unfold requires stable(p).satisfied_by(ex), but we provide nothing.
// SHOULD FAIL
proof fn test_boundary_unfold_without_stable<T>(ex: Execution<T>, p: TempPred<T>)
    ensures p.satisfied_by(ex) ==> forall |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    stable_unfold::<T>(ex, p);
}

}
