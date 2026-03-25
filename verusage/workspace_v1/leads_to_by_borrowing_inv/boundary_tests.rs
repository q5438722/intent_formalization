use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (from target) =====

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

// ===== Helper lemmas (axioms from target) =====

#[verifier::external_body]
proof fn instantiate_entailed_always<T>(ex: Execution<T>, i: nat, spec: TempPred<T>, p: TempPred<T>)
    requires
        spec.satisfied_by(ex),
        spec.implies(always(p)).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn instantiate_entailed_leads_to<T>(ex: Execution<T>, i: nat, spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.satisfied_by(ex),
        spec.implies(p.leads_to(q)).satisfied_by(ex),
    ensures p.implies(eventually(q)).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

// ===== Target function =====

pub proof fn leads_to_by_borrowing_inv<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, inv: TempPred<T>)
    requires
        spec.entails(p.and(inv).leads_to(q)),
        spec.entails(always(inv)),
    ensures
        spec.entails(p.leads_to(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(q).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.satisfied_by(ex.suffix(i)) implies eventually(q).satisfied_by(ex.suffix(i)) by {
            instantiate_entailed_always(ex, i, spec, inv);
            instantiate_entailed_leads_to(ex, i, spec, p.and(inv), q);
        }
    }
}

// ===== BOUNDARY TESTS =====

// Test 1: Missing always(inv) precondition
// Calls leads_to_by_borrowing_inv without spec.entails(always(inv))
// SHOULD FAIL
proof fn test_boundary_missing_always_inv<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, inv: TempPred<T>)
    requires
        spec.entails(p.and(inv).leads_to(q)),
        // MISSING: spec.entails(always(inv)),
    ensures
        spec.entails(p.leads_to(q)),
{
    leads_to_by_borrowing_inv(spec, p, q, inv);
}

// Test 2: Missing leads_to precondition
// Calls leads_to_by_borrowing_inv without spec.entails(p.and(inv).leads_to(q))
// SHOULD FAIL
proof fn test_boundary_missing_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, inv: TempPred<T>)
    requires
        // MISSING: spec.entails(p.and(inv).leads_to(q)),
        spec.entails(always(inv)),
    ensures
        spec.entails(p.leads_to(q)),
{
    leads_to_by_borrowing_inv(spec, p, q, inv);
}

// Test 3: Both preconditions missing
// Calls leads_to_by_borrowing_inv with no requires at all
// SHOULD FAIL
proof fn test_boundary_no_preconditions<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, inv: TempPred<T>)
    ensures
        spec.entails(p.leads_to(q)),
{
    leads_to_by_borrowing_inv(spec, p, q, inv);
}

// Test 4: instantiate_entailed_always without spec.satisfied_by(ex)
// SHOULD FAIL
proof fn test_boundary_always_no_spec<T>(ex: Execution<T>, i: nat, spec: TempPred<T>, p: TempPred<T>)
    requires
        // MISSING: spec.satisfied_by(ex),
        spec.implies(always(p)).satisfied_by(ex),
    ensures
        p.satisfied_by(ex.suffix(i)),
{
    instantiate_entailed_always(ex, i, spec, p);
}

// Test 5: instantiate_entailed_leads_to without spec.implies(p.leads_to(q)).satisfied_by(ex)
// SHOULD FAIL
proof fn test_boundary_leads_to_no_implies<T>(ex: Execution<T>, i: nat, spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.satisfied_by(ex),
        // MISSING: spec.implies(p.leads_to(q)).satisfied_by(ex),
    ensures
        p.implies(eventually(q)).satisfied_by(ex.suffix(i)),
{
    instantiate_entailed_leads_to(ex, i, spec, p, q);
}

}
