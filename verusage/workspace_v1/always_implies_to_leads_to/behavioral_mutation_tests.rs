use vstd::prelude::*;

fn main() {}

verus!{

// ===== Common Definitions (from target file) =====

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
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

pub proof fn always_implies_to_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(p.leads_to(q)),
{
    assert forall |ex| spec.satisfied_by(ex)
    implies #[trigger] p.leads_to(q).satisfied_by(ex) by {
        implies_apply(ex, spec, always(p.implies(q)));
        always_unfold(ex, p.implies(q));
        assert forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i))
        implies eventually(q).satisfied_by(ex.suffix(i)) by {
            implies_apply(ex.suffix(i), p, q);
            execution_equality::<T>(ex.suffix(i), ex.suffix(i).suffix(0));
        };
    };
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// ===== Behavioral Mutation Tests =====
// These tests start from valid preconditions but mutate the expected output/relation.
// Each test SHOULD FAIL verification.

// SHOULD FAIL
// Test 1: Swap p and q in the conclusion.
// From spec ⊨ □(p → q), we cannot derive q ~> p (the converse leads-to).
proof fn test_swapped_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(q.leads_to(p)),
{
    always_implies_to_leads_to(spec, p, q);
}

// SHOULD FAIL
// Test 2: Strengthen conclusion to always(q).
// From spec ⊨ □(p → q), we cannot derive spec ⊨ □q.
// The implication is conditional on p; q need not hold unconditionally.
proof fn test_always_q_from_always_piq<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(always(q)),
{
    always_implies_to_leads_to(spec, p, q);
}

// SHOULD FAIL
// Test 3: Reverse the theorem direction.
// From spec ⊨ p ~> q, we cannot derive spec ⊨ □(p → q).
// leads_to only guarantees eventually(q), not immediate q.
proof fn test_leads_to_to_always_implies<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(always(p.implies(q))),
{
}

// SHOULD FAIL
// Test 4: From leads_to, derive always(q).
// p ~> q only says "whenever p, eventually q". It does NOT imply q always holds.
proof fn test_always_q_from_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(always(q)),
{
}

// SHOULD FAIL
// Test 5: From spec ⊨ □(p → q), try to derive spec ⊨ ◇(□q).
// □(p → q) does not guarantee that q eventually holds always;
// it only guarantees q holds conditionally when p holds.
proof fn test_leads_to_implies_eventually_always<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(eventually(always(q))),
{
    always_implies_to_leads_to(spec, p, q);
}

}
