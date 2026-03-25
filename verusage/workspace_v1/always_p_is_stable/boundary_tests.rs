use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (copied from target) =====

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

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall|ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

// ===== Boundary Tests =====

// Test 1: Violate the precondition of always_propagate_forwards
// Calling with an arbitrary execution where always(p) may not hold.
// SHOULD FAIL
proof fn test_boundary_violate_precondition<T>(ex: Execution<T>, p: TempPred<T>, i: nat) {
    always_propagate_forwards::<T>(ex, p, i);
}

// Test 2: Try to conclude always(p) from p holding at a single point.
// Knowing p at one execution does NOT imply p at all suffixes.
// SHOULD FAIL
proof fn test_boundary_always_from_single_point<T>(ex: Execution<T>, p: TempPred<T>)
    requires
        p.satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex),
{
}

// Test 3: Try to conclude p from stable(p).
// stable(p) = (p => always(p)), which is vacuously true when p is false.
// So stable(p) does NOT imply p.
// SHOULD FAIL
proof fn test_boundary_stable_does_not_imply_p<T>(ex: Execution<T>, p: TempPred<T>)
    requires
        stable(p).satisfied_by(ex),
    ensures
        p.satisfied_by(ex),
{
}

}
