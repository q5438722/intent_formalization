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

// ========== Logical Tests ==========

// Test 1: leads_to does NOT imply unconditional eventually.
// From spec.entails(p.leads_to(q)), try to derive spec.entails(eventually(q)) without p ever holding.
// p.leads_to(q) = always(p ==> eventually(q)), but if p never holds, eventually(q) is not guaranteed.
// SHOULD FAIL
proof fn test_logical_leads_to_does_not_imply_eventually()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    assert(spec.entails(eventually(q)));
}

// Test 2: leads_to for an unrelated target predicate.
// p.entails(q) gives spec.entails(p.leads_to(q)), but NOT spec.entails(p.leads_to(r))
// where r is unrelated to q. With r = (state(0) == 42), p = (state(0) > 0):
// an execution with state(i) = 1 for all i satisfies p at position 0 but never satisfies r.
// SHOULD FAIL
proof fn test_logical_unrelated_target()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    let r = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 42);
    assert(spec.entails(p.leads_to(r)));
}

// Test 3: Cross-function misuse — derive a structurally unrelated global property.
// From spec.entails(p.leads_to(q)), try to assert spec entails always of an unrelated predicate
// (that consecutive states are equal). This is a structural/global assumption not guaranteed.
// SHOULD FAIL
proof fn test_logical_unrelated_global_property()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    entails_implies_leads_to(spec, p, q);
    let unrelated = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == (ex.nat_to_state)(1));
    assert(spec.entails(always(unrelated)));
}

}
