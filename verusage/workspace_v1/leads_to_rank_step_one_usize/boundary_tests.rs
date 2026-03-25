use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions (from source) ==========

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

// ========== Function Under Test ==========

#[verifier::external_body]
proof fn leads_to_rank_step_one_usize_help<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>, n: usize)
    requires
        forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),
    ensures
        spec.entails(p(n).leads_to(p(0))),
    decreases n,
{
    unimplemented!()
}

pub proof fn leads_to_rank_step_one_usize<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
    requires
        forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),
    ensures
        forall |n: usize| #[trigger] spec.entails(p(n).leads_to(p(0))),
{
    let pre = {
        forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize))))
    };
    assert forall |n: usize| pre implies #[trigger] spec.entails(p(n).leads_to(p(0))) by {
        leads_to_rank_step_one_usize_help(spec, p, n);
    }
}

// ========== BOUNDARY TESTS ==========

// Test 1: Call leads_to_rank_step_one_usize without any precondition
// SHOULD FAIL - requires clause is not satisfied
proof fn test_boundary_missing_precondition<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
{
    leads_to_rank_step_one_usize(spec, p);
}

// Test 2: Only provide the step-down property for n=1, not for all n > 0
// SHOULD FAIL - partial precondition is insufficient
proof fn test_boundary_partial_precondition<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
    requires
        spec.entails(p(1usize).leads_to(p(0usize))),
{
    leads_to_rank_step_one_usize(spec, p);
}

// Test 3: Weaken the precondition to n > 1 (skip the base step n=1 -> n=0)
// SHOULD FAIL - missing the crucial n=1 case
proof fn test_boundary_skip_base_step<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
    requires
        forall |n: usize| #![trigger p(n)] (n > 1 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),
{
    leads_to_rank_step_one_usize(spec, p);
}

}
