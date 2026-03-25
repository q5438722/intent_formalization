use vstd::prelude::*;

fn main() {}

verus!{

// ---- Definitions (from target file) ----

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

    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

spec fn eventually_choose_witness<T>(ex: Execution<T>, p: TempPred<T>) -> nat
    recommends exists |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    let witness = choose |i| p.satisfied_by(#[trigger] ex.suffix(i));
    witness
}

// ---- Logical Tests ----
// Properties NOT explicitly guaranteed by the specification.

// Test 1: always(eventually(p)) does NOT imply eventually(always(p))
// This is a well-known non-theorem in temporal logic (infinitely often ≠ eventually forever).
// SHOULD FAIL
proof fn logical_always_eventually_not_implies_eventually_always<T>(
    p: TempPred<T>, ex: Execution<T>,
)
    requires always(eventually(p)).satisfied_by(ex),
    ensures eventually(always(p)).satisfied_by(ex),
{
}

// Test 2: leads_to is NOT symmetric — p ↝ q does not imply q ↝ p
// SHOULD FAIL
proof fn logical_leads_to_not_symmetric<T>(
    p: TempPred<T>, q: TempPred<T>,
)
    requires valid(p.leads_to(q)),
    ensures valid(q.leads_to(p)),
{
}

// Test 3: eventually(p) ∧ eventually(q) does NOT imply eventually(p ∧ q)
// The two witnesses may occur at different times; simultaneous satisfaction is not guaranteed.
// SHOULD FAIL
proof fn logical_eventually_conjunction_invalid<T>(
    p: TempPred<T>, q: TempPred<T>, ex: Execution<T>,
)
    requires
        eventually(p).satisfied_by(ex),
        eventually(q).satisfied_by(ex),
    ensures
        eventually(p.and(q)).satisfied_by(ex),
{
}

// Test 4: choose-based witness is NOT guaranteed to equal a specific known index
// Even though p holds at both suffix(2) and suffix(5), we cannot assert which one choose picks.
// SHOULD FAIL
proof fn logical_witness_not_deterministic<T>(
    ex: Execution<T>, p: TempPred<T>,
)
    requires
        p.satisfied_by(ex.suffix(2)),
        p.satisfied_by(ex.suffix(5)),
    ensures
        eventually_choose_witness(ex, p) == 2nat,
{
}

}
