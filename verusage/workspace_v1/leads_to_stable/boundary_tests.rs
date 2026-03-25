use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions (from target) ==========

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

pub open spec fn later<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.satisfied_by(ex.suffix(1)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ========== Axioms (from target) ==========

#[verifier::external_body]
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
    ensures eventually(p).satisfied_by(ex)
{ unimplemented!() }

#[verifier::external_body]
proof fn next_preserves_inv_rec<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)),
    decreases i,
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// ========== BOUNDARY TESTS ==========
// Each test violates a precondition of an axiom or theorem.
// All tests SHOULD FAIL verification.

// SHOULD FAIL: implies_apply called without p.satisfied_by(ex)
proof fn test_implies_apply_missing_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    // Missing: p.satisfied_by(ex)
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: implies_apply called without p.implies(q).satisfied_by(ex)
proof fn test_implies_apply_missing_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    // Missing: p.implies(q).satisfied_by(ex)
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: always_propagate_forwards without always(p)
proof fn test_always_propagate_no_always<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires p.satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    // p holds at ex, but not always(p)
    always_propagate_forwards::<T>(ex, p, i);
}

// SHOULD FAIL: eventually_proved_by_witness with unsatisfied witness
proof fn test_witness_not_satisfied<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    ensures eventually(p).satisfied_by(ex),
{
    // Missing: p.satisfied_by(ex.suffix(witness_idx))
    eventually_proved_by_witness::<T>(ex, p, witness_idx);
}

// SHOULD FAIL: next_preserves_inv_rec missing base case
proof fn test_induction_no_base_case<T>(ex: Execution<T>, next: TempPred<T>, inv: TempPred<T>, i: nat)
    requires
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
    ensures inv.satisfied_by(ex.suffix(i)),
{
    // Missing: inv.satisfied_by(ex)
    next_preserves_inv_rec::<T>(ex, next, inv, i);
}

// SHOULD FAIL: leads_to_stable missing stability preservation condition
proof fn test_leads_to_stable_missing_stability<T>(spec: TempPred<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        // Missing: spec.entails(always(q.and(next).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(p.leads_to(q)),
    ensures spec.entails(p.leads_to(always(q))),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(always(q)).satisfied_by(ex) by {
        // Cannot proceed without stability condition
    };
}

// SHOULD FAIL: execution_equality with no pointwise equality guarantee
proof fn test_execution_equality_no_evidence<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    // Missing: forall |i: nat| (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i)
    execution_equality::<T>(ex1, ex2);
}

// SHOULD FAIL: leads_to_unfold without leads_to holding
proof fn test_leads_to_unfold_no_precondition<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    ensures forall |i: nat| p.implies(eventually(q)).satisfied_by(#[trigger] ex.suffix(i)),
{
    // Missing: p.leads_to(q).satisfied_by(ex)
    leads_to_unfold::<T>(ex, p, q);
}

}
