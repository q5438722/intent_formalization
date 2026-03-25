use vstd::prelude::*;

fn main() {}

verus!{

// =====================================================================
// Definitions from target file
// =====================================================================

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

// =====================================================================
// Functions under test
// =====================================================================

#[verifier::external_body]
proof fn leads_to_rank_step_one_help<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>, n: nat)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
    ensures
        spec.entails(p(n).leads_to(p(0))),
    decreases n,
{
    unimplemented!()
}

pub proof fn leads_to_rank_step_one<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
    ensures
        forall |n: nat| #[trigger] spec.entails(p(n).leads_to(p(0))),
{
    let pre = {
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat))))
    };
    assert forall |n: nat| pre implies #[trigger] spec.entails(p(n).leads_to(p(0))) by {
        leads_to_rank_step_one_help(spec, p, n);
    }
}

// =====================================================================
// BOUNDARY TESTS — violate preconditions, use edge cases
// =====================================================================

// Test 1: Call leads_to_rank_step_one with NO precondition at all
// SHOULD FAIL: requires clause is not satisfied
proof fn test_boundary_no_precondition<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
{
    leads_to_rank_step_one(spec, p); // SHOULD FAIL
}

// Test 2: Satisfy precondition for only ONE specific n, not forall
// SHOULD FAIL: a single case does not satisfy the universal quantifier
proof fn test_boundary_partial_precondition<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        spec.entails(p(1).leads_to(p(0))),
{
    leads_to_rank_step_one(spec, p); // SHOULD FAIL
}

// Test 3: Provide precondition with REVERSED direction (p(n-1) leads_to p(n))
// SHOULD FAIL: direction mismatch with requires
proof fn test_boundary_reversed_direction<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p((n - 1) as nat).leads_to(p(n)))),
{
    leads_to_rank_step_one(spec, p); // SHOULD FAIL
}

}
