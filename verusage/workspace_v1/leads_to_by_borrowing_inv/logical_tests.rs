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

// ===== LOGICAL TESTS =====
// Properties NOT explicitly guaranteed by the spec.
// These probe structural/logical assumptions that go beyond stated behavior.

// Test 1: leads_to symmetry — p leads_to q does NOT imply q leads_to p
// SHOULD FAIL
proof fn test_logical_symmetry<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(q.leads_to(p)), // Symmetry: NOT a valid temporal logic law
{
}

// Test 2: leads_to conjunction — p→◇q and p→◇r does NOT imply p→◇(q∧r)
// q and r may hold at different future times
// SHOULD FAIL
proof fn test_logical_conjunction<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(p.leads_to(r)),
    ensures
        spec.entails(p.leads_to(q.and(r))), // NOT derivable: q and r may hold at different times
{
}

// Test 3: leads_to strengthening to always — eventually(q) does NOT imply always(q)
// SHOULD FAIL
proof fn test_logical_eventually_to_always<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(p.leads_to(always(q))), // q holding eventually ≠ q holding forever
{
}

// Test 4: Invariant sufficiency — always(inv) alone does NOT imply p leads_to q
// SHOULD FAIL
proof fn test_logical_invariant_sufficiency<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, inv: TempPred<T>)
    requires
        spec.entails(always(inv)),
        // MISSING: spec.entails(p.and(inv).leads_to(q))
    ensures
        spec.entails(p.leads_to(q)), // Invariant alone cannot establish liveness
{
}

}
