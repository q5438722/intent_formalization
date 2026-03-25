use vstd::prelude::*;

fn main() {}

verus!{

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

// ===== Axioms (from target) =====

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
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex),
{ unimplemented!() }

spec fn eventually_choose_witness<T>(ex: Execution<T>, p: TempPred<T>) -> nat
    recommends exists |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    let witness = choose |i| p.satisfied_by(#[trigger] ex.suffix(i));
    witness
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// ===== Main lemma (from target) =====

pub proof fn leads_to_framed_by_or<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(p.or(r).leads_to(q.or(r))),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.or(r).leads_to(q.or(r)).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.or(r).satisfied_by(ex.suffix(i)) implies eventually(q.or(r)).satisfied_by(ex.suffix(i)) by {
            implies_apply(ex, spec, p.leads_to(q));
            leads_to_unfold(ex, p, q);
            if p.satisfied_by(ex.suffix(i)) {
                implies_apply(ex.suffix(i), p, eventually(q));
                let witness_idx = eventually_choose_witness(ex.suffix(i), q);
                eventually_proved_by_witness(ex.suffix(i), q.or(r), witness_idx);
            } else {
                let witness_idx = 0;
                execution_equality(ex.suffix(i), ex.suffix(i).suffix(0));
                eventually_proved_by_witness(ex.suffix(i), q.or(r), witness_idx);
            }
        }
    }
}

// ===== BOUNDARY TESTS =====

// Test 1: Assert the conclusion without any precondition
// The framing property should NOT hold for arbitrary spec, p, q, r
// SHOULD FAIL
proof fn test_boundary_1_no_precondition<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    ensures spec.entails(p.or(r).leads_to(q.or(r))),
{
    // No assumption that spec.entails(p.leads_to(q))
    // This should fail — the conclusion is not a tautology
}

// Test 2: Call leads_to_framed_by_or without satisfying its requires clause
// Tests that the precondition is actually enforced
// SHOULD FAIL
proof fn test_boundary_2_call_without_requires<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
{
    // spec.entails(p.leads_to(q)) is NOT established
    // The call should be rejected by the verifier
    leads_to_framed_by_or(spec, p, q, r);
}

// Test 3: Use implies_apply without the implication holding
// Tests that modus ponens requires the implication as a precondition
// SHOULD FAIL
proof fn test_boundary_3_implies_without_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    // We have p.satisfied_by(ex) but NOT p.implies(q).satisfied_by(ex)
    // implies_apply should not be callable
    implies_apply(ex, p, q);
}

// Test 4: Use eventually_proved_by_witness without the witness condition
// Tests that the witness must actually satisfy the predicate
// SHOULD FAIL
proof fn test_boundary_4_eventually_without_witness<T>(ex: Execution<T>, p: TempPred<T>)
    ensures eventually(p).satisfied_by(ex),
{
    // p.satisfied_by(ex.suffix(0)) is NOT established
    // The call should be rejected
    eventually_proved_by_witness(ex, p, 0);
}

}
