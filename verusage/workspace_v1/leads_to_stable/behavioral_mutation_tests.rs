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

// ========== BEHAVIORAL MUTATION TESTS ==========
// Each test starts from valid inputs but asserts a WRONG output relation.
// All tests SHOULD FAIL verification.

// SHOULD FAIL: eventually(p) does NOT imply always(p)
proof fn test_eventually_implies_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
    // eventually means "exists some i where p holds"
    // always means "forall i, p holds" — strictly stronger
    // This should not be provable.
}

// SHOULD FAIL: leads_to is NOT symmetric (p ~> q does NOT imply q ~> p)
proof fn test_leads_to_not_symmetric<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.leads_to(q).satisfied_by(ex),
    ensures q.leads_to(p).satisfied_by(ex),
{
    // p leads to q does not mean q leads to p
    leads_to_unfold::<T>(ex, p, q);
}

// SHOULD FAIL: always(p => q) does NOT imply always(q => p)
proof fn test_implication_not_symmetric<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires always(p.implies(q)).satisfied_by(ex),
    ensures always(q.implies(p)).satisfied_by(ex),
{
    // Implication is not symmetric
}

// SHOULD FAIL: leads_to_stable conclusion should NOT be always(p AND q), only always(q)
proof fn test_leads_to_stable_stronger_conclusion<T>(
    spec: TempPred<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec.entails(always(q.and(next).implies(later(q)))),
        spec.entails(always(next)),
        spec.entails(p.leads_to(q)),
    ensures spec.entails(p.leads_to(always(p.and(q)))),
{
    // The real conclusion is p.leads_to(always(q)), not p.leads_to(always(p AND q))
    // p may not persist, so p AND q is too strong
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(always(p.and(q))).satisfied_by(ex) by {
    };
}

// SHOULD FAIL: p.leads_to(q) does NOT imply p.leads_to(always(q)) without stability
proof fn test_leads_to_without_stability<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
    ensures spec.entails(p.leads_to(always(q))),
{
    // Without the stability condition (q AND next => later(q)),
    // q might hold temporarily then stop. leads_to(always(q)) should not follow.
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(always(q)).satisfied_by(ex) by {
    };
}

// SHOULD FAIL: always(p) and always(q) together do NOT imply always(p.leads_to(q))
// (leads_to involves eventually, which is structurally different)
proof fn test_always_pair_not_leads_to<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p).satisfied_by(ex),
    ensures p.leads_to(q).satisfied_by(ex),
{
    // Knowing p always holds says nothing about whether q eventually holds
}

// SHOULD FAIL: the witness for eventually must be exact; shifting by 1 breaks it
proof fn test_witness_off_by_one<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex.suffix(1)),
{
    // The witness is relative to ex, not ex.suffix(1)
    // We can't just shift the execution without adjusting the witness
    eventually_proved_by_witness::<T>(ex.suffix(1), p, witness_idx);
}

}
