use vstd::prelude::*;

fn main() {}

verus!{

// === Type Definitions (from source) ===

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

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
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

// === Axioms (from source) ===

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
pub proof fn leads_to_trans<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(p.leads_to(r)),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn leads_to_framed_by_or<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(p.or(r).leads_to(q.or(r))),
{ unimplemented!() }

pub proof fn leads_to_shortcut_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>)
    requires
        spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),
    ensures spec.entails(p.leads_to(r.or(s))),
{
    leads_to_framed_by_or(spec, q, r.or(s), s);
    temp_pred_equality(r.or(s).or(s), r.or(s));
    leads_to_trans(spec, p, q.or(s), r.or(s));
}

// ========================
// === LOGICAL TESTS ===
// ========================

// Test 1: leads_to is NOT symmetric — p ~> q does not imply q ~> p
// SHOULD FAIL
proof fn logical_leads_to_not_symmetric<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(q.leads_to(p)),
{
}

// Test 2: Disjunct elimination — p ~> (q ∨ s) does NOT imply p ~> q
// SHOULD FAIL
proof fn logical_or_elimination<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, s: TempPred<T>
)
    requires spec.entails(p.leads_to(q.or(s))),
    ensures spec.entails(p.leads_to(q)),
{
}

// Test 3: Strengthened shortcut — from valid shortcut premises, p ~> r should NOT follow
// (the spec only guarantees p ~> (r ∨ s), not p ~> r)
// SHOULD FAIL
proof fn logical_strengthen_shortcut<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),
    ensures spec.entails(p.leads_to(r)),
{
    leads_to_shortcut_temp(spec, p, q, r, s);
}

// Test 4: leads_to does NOT imply immediate implication
// p ~> q means always(p => eventually(q)), NOT always(p => q)
// SHOULD FAIL
proof fn logical_leads_to_not_immediate<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(always(p.implies(q))),
{
}

// Test 5: entailment does NOT distribute over disjunction
// spec ⊨ (p ∨ q) does NOT imply spec ⊨ p
// SHOULD FAIL
proof fn logical_entails_not_distribute_or<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires spec.entails(p.or(q)),
    ensures spec.entails(p),
{
}

}
