use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from source =====

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

    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
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

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// ===== Boundary Tests =====

// Test 1: Prove eventually(p) without any witness or precondition.
// Without a concrete witness index, eventually should not be provable.
// SHOULD FAIL
proof fn test_eventually_no_witness<T>(ex: Execution<T>, p: TempPred<T>)
    ensures eventually(p).satisfied_by(ex),
{
    // No witness provided — cannot establish "exists i such that p holds at suffix(i)"
}

// Test 2: Prove always(p) from p holding at only one suffix (suffix(0)).
// always requires p at ALL suffixes, not just one.
// SHOULD FAIL
proof fn test_always_from_single_point<T>(ex: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex.suffix(0)),
    ensures always(p).satisfied_by(ex),
{
    // Only know p holds at suffix(0), not at all suffixes
}

// Test 3: Prove valid(always(p)) for an arbitrary predicate.
// Not all predicates hold always on all executions.
// SHOULD FAIL
proof fn test_valid_always_arbitrary<T>(p: TempPred<T>)
    ensures valid(always(p)),
{
    // Arbitrary p does not hold always
}

}
