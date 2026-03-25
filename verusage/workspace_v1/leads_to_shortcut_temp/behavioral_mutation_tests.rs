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

// ====================================
// === BEHAVIORAL MUTATION TESTS ===
// ====================================

// Test 1: Mutate conclusion — drop or(s), claim p ~> r (strictly stronger)
// SHOULD FAIL
proof fn mutation_drop_disjunct<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),
    ensures spec.entails(p.leads_to(r)),
{
    leads_to_shortcut_temp(spec, p, q, r, s);
}

// Test 2: Mutate conclusion — claim p ~> q (wrong target)
// SHOULD FAIL
proof fn mutation_wrong_target<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),
    ensures spec.entails(p.leads_to(q)),
{
    leads_to_shortcut_temp(spec, p, q, r, s);
}

// Test 3: Mutate conclusion — reverse direction, claim (r∨s) ~> p
// SHOULD FAIL
proof fn mutation_reverse_direction<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, s: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q.or(s))),
        spec.entails(q.leads_to(r.or(s))),
    ensures spec.entails(r.or(s).leads_to(p)),
{
    leads_to_shortcut_temp(spec, p, q, r, s);
}

// Test 4: Mutate leads_to_trans conclusion — from p ~> q and q ~> r, claim r ~> p
// SHOULD FAIL
proof fn mutation_trans_reverse<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(r.leads_to(p)),
{
    leads_to_trans(spec, p, q, r);
}

// Test 5: Mutate leads_to_framed_by_or conclusion — drop frame from conclusion
// From spec ⊨ p ~> q, claim spec ⊨ (p∨r) ~> q (instead of (p∨r) ~> (q∨r))
// SHOULD FAIL
proof fn mutation_framed_or_drop_frame<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(p.or(r).leads_to(q)),
{
    leads_to_framed_by_or(spec, p, q, r);
}

}
