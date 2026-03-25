use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions from target file ==========

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

#[verifier::external_body]
proof fn valid_p_implies_always_p<T>(p: TempPred<T>)
    requires valid(p),
    ensures valid(always(p)),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn always_implies_to_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(p.leads_to(q)),
{
    unimplemented!()
}

pub proof fn entails_implies_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures spec.entails(p.leads_to(q)),
{
    valid_p_implies_always_p(p.implies(q));
    always_implies_to_leads_to(spec, p, q);
}

// ========== Behavioral Mutation Tests ==========

// Test 1: Reversed leads_to direction.
// p.entails(q) holds (false ==> true is valid), but q.leads_to(p) should NOT hold.
// q.leads_to(p) = always(q.implies(eventually(p))) = always(true ==> eventually(false)) = always(eventually(false))
// which is false for any execution since eventually(false) is never satisfied.
// SHOULD FAIL
proof fn test_mutation_reversed_leads_to()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    assert(spec.entails(q.leads_to(p)));
}

// Test 2: Converse entails — from p.entails(q) try to conclude q.entails(p).
// p = (state(0) > 0), q = (state(0) >= 0). p.entails(q) holds (x > 0 ==> x >= 0),
// but q.entails(p) does NOT hold (x >= 0 does not imply x > 0; x = 0 is a counterexample).
// SHOULD FAIL
proof fn test_mutation_converse_entails()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    assert(q.entails(p));
}

// Test 3: Strengthen conclusion from leads_to to always(q).
// p.entails(q) gives us spec.entails(p.leads_to(q)), but NOT spec.entails(always(q)).
// always(q) = for all i, (state(i) >= 0), which is false for executions with negative states.
// SHOULD FAIL
proof fn test_mutation_strengthen_to_always()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    assert(spec.entails(always(q)));
}

}
