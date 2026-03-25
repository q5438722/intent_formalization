// Logical Tests for leads_to_weaken
// Each test probes properties NOT explicitly guaranteed by the specification.
// All tests SHOULD FAIL verification.

use vstd::prelude::*;

fn main() {}

verus! {

// ── Definitions (from source) ──────────────────────────────────────

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
pub proof fn always_implies_to_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(p.leads_to(q)),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn leads_to_trans<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(p.leads_to(r)),
{ unimplemented!() }

pub proof fn leads_to_weaken<T>(spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(p2.leads_to(q2)),
{
    always_implies_to_leads_to::<T>(spec, p2, p1);
    always_implies_to_leads_to::<T>(spec, q1, q2);
    leads_to_trans::<T>(spec, p2, p1, q1);
    leads_to_trans::<T>(spec, p2, q1, q2);
}

// ── Logical Tests ──────────────────────────────────────────────────

// LT1: Symmetry of leads_to is NOT guaranteed.
// If p ~> q, it does NOT follow that q ~> p.
// SHOULD FAIL
proof fn lt1_leads_to_not_symmetric<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(q.leads_to(p)),  // NOT guaranteed
{
}

// LT2: leads_to does NOT imply always.
// p ~> q does NOT mean always(q).
// SHOULD FAIL
proof fn lt2_leads_to_not_implies_always<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(always(q)),  // NOT guaranteed
{
}

// LT3: Weakening with identity implications should NOT strengthen.
// Even with p2 = p1 (via entails), leads_to should not imply always.
// SHOULD FAIL
proof fn lt3_weaken_identity_no_strengthen<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec.entails(always(p.implies(p))),
        spec.entails(always(q.implies(q))),
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(always(p.implies(q))),  // NOT guaranteed: leads_to != always-implies
{
    leads_to_weaken::<T>(spec, p, q, p, q);
}

// LT4: Cross-function misuse: using leads_to_trans without valid chain.
// Attempting to derive p ~> r from p ~> q alone (no q ~> r).
// SHOULD FAIL
proof fn lt4_trans_without_second_link<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
        // MISSING: spec.entails(q.leads_to(r)),
    ensures
        spec.entails(p.leads_to(r)),  // NOT guaranteed
{
    leads_to_trans::<T>(spec, p, q, r);
}

// LT5: Entailment is NOT symmetric.
// spec.entails(p) does NOT imply p.entails(spec).
// SHOULD FAIL
proof fn lt5_entails_not_symmetric<T>(
    spec: TempPred<T>, p: TempPred<T>
)
    requires
        spec.entails(p),
    ensures
        p.entails(spec),  // NOT guaranteed
{
}

// LT6: Cannot derive leads_to for unrelated predicates.
// Having p ~> q gives no information about r ~> s for unrelated r, s.
// SHOULD FAIL
proof fn lt6_unrelated_leads_to<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(r.leads_to(s)),  // NOT guaranteed for unrelated r, s
{
}

}
