use vstd::prelude::*;

fn main() {}

verus!{

// ============================================================
// Common Definitions (from target: wf1_variant_temp.rs)
// ============================================================

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

pub open spec fn later<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.satisfied_by(ex.suffix(1)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ============================================================
// Trusted Axioms (external_body from target)
// ============================================================

#[verifier::external_body]
#[verifier::spinoff_prover]
proof fn leads_to_unfold<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.leads_to(q).satisfied_by(ex),
    ensures forall |i: nat| p.implies(eventually(q)).satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn always_p_or_eventually_q<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
    ensures always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

#[verifier::external_body]
pub proof fn wf1_variant_temp<T>(spec: TempPred<T>, next: TempPred<T>, forward: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(always(p).leads_to(forward)),
    ensures spec.entails(p.leads_to(q)),
{ unimplemented!() }


// ============================================================
// BOUNDARY TESTS: Violate preconditions of spec functions
// ============================================================

// SHOULD FAIL: Missing 1st precondition (stuttering preservation)
proof fn boundary_missing_stuttering_preservation(
    spec: TempPred<int>, next: TempPred<int>, forward: TempPred<int>,
    p: TempPred<int>, q: TempPred<int>
)
    requires
        // DROPPED: spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(always(p).leads_to(forward)),
{
    wf1_variant_temp(spec, next, forward, p, q);
}

// SHOULD FAIL: Missing 2nd precondition (forward implies progress)
proof fn boundary_missing_forward_progress(
    spec: TempPred<int>, next: TempPred<int>, forward: TempPred<int>,
    p: TempPred<int>, q: TempPred<int>
)
    requires
        spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        // DROPPED: spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(always(p).leads_to(forward)),
{
    wf1_variant_temp(spec, next, forward, p, q);
}

// SHOULD FAIL: Missing 3rd precondition (always next)
proof fn boundary_missing_always_next(
    spec: TempPred<int>, next: TempPred<int>, forward: TempPred<int>,
    p: TempPred<int>, q: TempPred<int>
)
    requires
        spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        // DROPPED: spec.entails(always(next)),
        spec.entails(always(p).leads_to(forward)),
{
    wf1_variant_temp(spec, next, forward, p, q);
}

// SHOULD FAIL: Missing 4th precondition (liveness: always p leads to forward)
proof fn boundary_missing_liveness(
    spec: TempPred<int>, next: TempPred<int>, forward: TempPred<int>,
    p: TempPred<int>, q: TempPred<int>
)
    requires
        spec.entails(always(p.and(next).implies(later(p).or(later(q))))),
        spec.entails(always(p.and(next).and(forward).implies(later(q)))),
        spec.entails(always(next)),
        // DROPPED: spec.entails(always(p).leads_to(forward)),
{
    wf1_variant_temp(spec, next, forward, p, q);
}

// SHOULD FAIL: implies_apply called without antecedent p being satisfied
proof fn boundary_implies_apply_no_antecedent(
    ex: Execution<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        p.implies(q).satisfied_by(ex),
        // MISSING: p.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// SHOULD FAIL: eventually_proved_by_witness called without witness condition
proof fn boundary_eventually_no_witness(
    ex: Execution<int>, p: TempPred<int>, witness_idx: nat
)
    // No requirement that p.satisfied_by(ex.suffix(witness_idx))
{
    eventually_proved_by_witness(ex, p, witness_idx);
}

}
