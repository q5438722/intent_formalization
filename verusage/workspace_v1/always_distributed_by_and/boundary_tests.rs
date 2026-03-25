use vstd::prelude::*;

fn main() {}

verus! {

// ===================== Base Definitions =====================

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

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

// ===================== Boundary Tests =====================

// Test 1: Call always_unfold without establishing its precondition
// SHOULD FAIL — precondition `always(p).satisfied_by(ex)` is not met
proof fn test_always_unfold_missing_precondition<T>(ex: Execution<T>, p: TempPred<T>)
{
    always_unfold(ex, p); // requires always(p).satisfied_by(ex) — not provided
}

// Test 2: Try to prove valid(always(p)) for an arbitrary predicate p
// SHOULD FAIL — arbitrary predicates are not always true
proof fn test_always_holds_for_arbitrary_predicate<T>(p: TempPred<T>)
    ensures valid(always(p)),
{
    // No proof body — this should not be provable
}

// Test 3: Try to derive the always-unfolded result without any assumption
// SHOULD FAIL — cannot derive forall |i| p.satisfied_by(ex.suffix(i)) from nothing
proof fn test_derive_forall_suffix_from_nothing<T>(ex: Execution<T>, p: TempPred<T>)
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    // No precondition, no proof — should fail
}

}
