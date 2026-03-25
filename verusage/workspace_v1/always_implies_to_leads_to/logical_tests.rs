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

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// ===== Logical Tests =====
// These test properties NOT explicitly guaranteed by the specification.
// Each test SHOULD FAIL verification.

// SHOULD FAIL
// Test 1: A false predicate should not be valid.
// If this passes, the axioms are unsound.
proof fn test_false_is_valid()
    ensures valid(TempPred::<int>::new(|ex: Execution<int>| false)),
{
}

// SHOULD FAIL
// Test 2: Entailment is NOT symmetric.
// p.entails(q) does not imply q.entails(p) in general.
proof fn test_entailment_symmetry<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures q.entails(p),
{
}

// SHOULD FAIL
// Test 3: eventually(p) does NOT imply always(p).
// Existence of one state satisfying p does not mean all states satisfy p.
proof fn test_eventually_implies_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
}

// SHOULD FAIL
// Test 4: Two arbitrary executions should NOT be equal.
// Tests that execution_equality cannot be abused without the pointwise premise.
proof fn test_arbitrary_execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
}

// SHOULD FAIL
// Test 5: Leads-to transitivity should NOT be provable without an explicit proof.
// p ~> q and q ~> r does NOT automatically give p ~> r without a manual proof step.
proof fn test_leads_to_transitivity_unproven<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(p.leads_to(r)),
{
}

}
