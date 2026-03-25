// Behavioral Mutation Tests for leads_to_weaken
// Each test starts from valid inputs but asserts a MUTATED conclusion.
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

// ── Behavioral Mutation Tests ──────────────────────────────────────

// BM1: Mutate conclusion to reverse leads_to direction (q2 ~> p2)
// SHOULD FAIL
proof fn bm1_reversed_leads_to<T>(
    spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(q2.leads_to(p2)),  // MUTATED: reversed
{
    leads_to_weaken::<T>(spec, p1, q1, p2, q2);
}

// BM2: Mutate conclusion to stronger property: always(p2 => q2)
// SHOULD FAIL
proof fn bm2_strengthen_to_always_implies<T>(
    spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(always(p2.implies(q2))),  // MUTATED: stronger than leads_to
{
    leads_to_weaken::<T>(spec, p1, q1, p2, q2);
}

// BM3: Mutate conclusion to swap both predicates: q1 ~> p2
// q1.leads_to(p2) is NOT derivable from the premises.
// SHOULD FAIL
proof fn bm3_swapped_predicates<T>(
    spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(q1.leads_to(p2)),  // MUTATED: unrelated direction
{
    leads_to_weaken::<T>(spec, p1, q1, p2, q2);
}

// BM4: Mutate conclusion predicate: q2 ~> p1 instead of p2 ~> q2
// q2.leads_to(p1) is NOT derivable from the premises.
// SHOULD FAIL
proof fn bm4_reversed_wrong_predicates<T>(
    spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(q2.leads_to(p1)),  // MUTATED: completely wrong
{
    leads_to_weaken::<T>(spec, p1, q1, p2, q2);
}

// BM5: Mutate conclusion to entail always(q2): universally q2 holds
// SHOULD FAIL
proof fn bm5_strengthen_to_always_q2<T>(
    spec: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
    ensures spec.entails(always(q2)),  // MUTATED: much stronger
{
    leads_to_weaken::<T>(spec, p1, q1, p2, q2);
}

}
