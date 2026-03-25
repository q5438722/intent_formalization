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

// ===== Boundary Tests =====
// These tests violate preconditions or use invalid inputs.
// Each test SHOULD FAIL verification.

// SHOULD FAIL
// Test 1: Missing the `always` wrapper in the precondition.
// The lemma requires spec.entails(always(p.implies(q))), but we only provide
// spec.entails(p.implies(q)) — a strictly weaker condition.
proof fn test_missing_always_wrapper<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.implies(q)),
    ensures spec.entails(p.leads_to(q)),
{
    always_implies_to_leads_to(spec, p, q);
}

// SHOULD FAIL
// Test 2: No precondition at all.
// Tries to derive spec.entails(p.leads_to(q)) from nothing.
proof fn test_no_precondition<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    ensures spec.entails(p.leads_to(q)),
{
}

// SHOULD FAIL
// Test 3: Calling always_unfold on a predicate not wrapped in `always`.
// always_unfold requires always(p).satisfied_by(ex), but we only have p.satisfied_by(ex).
proof fn test_always_unfold_on_non_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold(ex, p);
}

// SHOULD FAIL
// Test 4: Calling implies_apply without the antecedent p holding.
// implies_apply requires both p.implies(q).satisfied_by(ex) AND p.satisfied_by(ex).
// We only provide the implication, not that p actually holds.
proof fn test_implies_apply_missing_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// SHOULD FAIL
// Test 5: Calling execution_equality without proving pointwise equality.
// execution_equality requires ∀i. ex1.nat_to_state(i) == ex2.nat_to_state(i),
// but we provide no such guarantee.
proof fn test_execution_equality_arbitrary<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality::<T>(ex1, ex2);
}

}
