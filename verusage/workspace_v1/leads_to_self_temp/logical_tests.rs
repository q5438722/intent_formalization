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

// ===== Logical Tests =====

// Test 1: All executions are equal — structural assumption not in spec.
// execution_equality requires pointwise state equality; arbitrary executions are not equal.
// SHOULD FAIL
proof fn test_all_executions_equal<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    // Cannot prove without the pointwise equality precondition
}

// Test 2: eventually(p) implies always(p) — a strictly stronger claim.
// Eventually means "at some point"; always means "at all points". These are different.
// SHOULD FAIL
proof fn test_eventually_implies_always<T>(p: TempPred<T>)
    ensures valid(eventually(p).implies(always(p))),
{
    // eventually =/=> always
}

// Test 3: leads_to is symmetric — if p ~> q then q ~> p.
// leads_to is NOT symmetric in general (p may cause q but q need not cause p).
// SHOULD FAIL
proof fn test_leads_to_symmetric<T>(p: TempPred<T>, q: TempPred<T>)
    requires valid(p.leads_to(q)),
    ensures valid(q.leads_to(p)),
{
    // Symmetry is not a valid property of leads_to
}

}
