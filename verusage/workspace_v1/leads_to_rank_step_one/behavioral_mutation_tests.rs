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
// BEHAVIORAL MUTATION TESTS — correct inputs, mutated outputs/relations
// =====================================================================

// Test 1: Reverse the leads_to direction — assert p(0) leads_to p(5)
// SHOULD FAIL: the postcondition gives p(n) leads_to p(0), NOT p(0) leads_to p(n)
proof fn test_mutation_reverse_direction<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
{
    leads_to_rank_step_one(spec, p);
    assert(spec.entails(p(0).leads_to(p(5)))); // SHOULD FAIL
}

// Test 2: Wrong target rank — assert p(5) leads_to p(1) instead of p(0)
// SHOULD FAIL: postcondition only guarantees leads_to p(0), not p(1)
proof fn test_mutation_wrong_target<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
{
    leads_to_rank_step_one(spec, p);
    assert(spec.entails(p(5).leads_to(p(1)))); // SHOULD FAIL
}

// Test 3: Assert universal intermediate target — for all n > 0, p(n) leads_to p(1)
// SHOULD FAIL: postcondition only guarantees p(n) leads_to p(0), not p(1)
proof fn test_mutation_universal_wrong_target<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
{
    leads_to_rank_step_one(spec, p);
    assert(forall |n: nat| n > 0 ==> #[trigger] spec.entails(p(n).leads_to(p(1)))); // SHOULD FAIL
}

}
