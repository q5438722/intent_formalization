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

// ===== LOGICAL TESTS =====

// Test 1: Drop the spec guard — entailment does NOT imply universal validity
// From spec.entails(p.leads_to(q)), try to prove valid(p.leads_to(q))
// This is WRONG: entails is relativized to spec; valid is universal
// SHOULD FAIL
proof fn test_logical_1_entails_to_valid<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures valid(p.leads_to(q)),
{
    // spec.entails(X) means: for all ex, spec(ex) ==> X(ex)
    // valid(X) means: for all ex, X(ex)
    // The latter is strictly stronger — it drops the spec guard
}

// Test 2: Symmetry — leads_to is NOT symmetric
// From spec.entails(p.leads_to(q)), try to prove spec.entails(q.leads_to(p))
// SHOULD FAIL
proof fn test_logical_2_symmetry<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(q.leads_to(p)),
{
    // p leads_to q does NOT imply q leads_to p
    // There is no reverse liveness guarantee
}

// Test 3: leads_to does NOT imply always
// From spec.entails(p.leads_to(q)), try to prove spec.entails(always(q))
// This is WRONG: leads_to says IF p THEN eventually q, not that q holds always
// SHOULD FAIL
proof fn test_logical_3_leads_to_implies_always<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(always(q)),
{
    // p.leads_to(q) = always(p ==> eventually(q))
    // This does NOT imply always(q) — q only holds eventually after p
}

// Test 4: Converse of framing rule — NOT valid
// From spec.entails(p.or(r).leads_to(q.or(r))), try to prove spec.entails(p.leads_to(q))
// This is WRONG: the framing rule is one-directional
// SHOULD FAIL
proof fn test_logical_4_converse_framing<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires spec.entails(p.or(r).leads_to(q.or(r))),
    ensures spec.entails(p.leads_to(q)),
{
    // Knowing p∨r leads to q∨r does NOT allow us to conclude p leads to q
    // The r component could be doing all the work
}

}
