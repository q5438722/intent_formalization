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

// ===== Logical Tests =====

// Logical Test 1: valid(stable(p)) does NOT imply valid(p)
// Stability says "if p holds, it persists." It does NOT say p holds everywhere.
// A predicate that never holds is trivially stable.
// SHOULD FAIL
proof fn test_logical_stable_does_not_imply_valid<T>(p: TempPred<T>)
    requires valid(stable(p)),
    ensures valid(p),
{
}

// Logical Test 2: Not every predicate is stable
// An arbitrary predicate may hold at one point and not at a later point.
// SHOULD FAIL
proof fn test_logical_any_pred_is_stable<T>(p: TempPred<T>)
    ensures valid(stable(p)),
{
}

// Logical Test 3: Stable conjunction does NOT decompose
// valid(stable(p.and(q))) does NOT imply valid(stable(p)).
// Stability of p∧q only triggers when both hold; it says nothing about p alone.
// SHOULD FAIL
proof fn test_logical_stable_conjunction_decomposition<T>(p: TempPred<T>, q: TempPred<T>)
    requires valid(stable(p.and(q))),
    ensures valid(stable(p)),
{
}

}
