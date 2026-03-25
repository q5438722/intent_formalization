use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type Definitions =====

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

// ===== Axioms =====

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn eventually_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures exists |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn entails_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn eventually_propagate_backwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires eventually(p).satisfied_by(ex.suffix(i)),
    ensures eventually(p).satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// =========================================================
// LOGICAL TESTS: Unintended reasoning / unwarranted properties
// =========================================================

// Test 1: Symmetry — leads_to is NOT symmetric
// SHOULD FAIL
proof fn test_logical_symmetry<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.leads_to(q)),
    ensures spec.entails(q.leads_to(p)),
{
    // leads_to is not symmetric: p ~> q does NOT imply q ~> p
}

// Test 2: Eventually does NOT imply Always
// SHOULD FAIL
proof fn test_logical_eventually_to_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
    // exists i. p(suffix(i)) does NOT imply forall i. p(suffix(i))
}

// Test 3: leads_to does NOT yield eventually(q) without the antecedent p
// SHOULD FAIL
proof fn test_logical_leads_to_no_antecedent<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        p.leads_to(q).satisfied_by(ex),
        // NOTE: p.satisfied_by(ex) is intentionally NOT required
    ensures eventually(q).satisfied_by(ex),
{
    // always(p => eventually(q)) does NOT imply eventually(q)
    // unless p actually holds at some point
}

// Test 4: always(p) at a single execution does NOT imply valid(p)
// SHOULD FAIL
proof fn test_logical_always_not_valid<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures valid(p),
{
    // forall i. p(ex.suffix(i)) does NOT mean forall ex'. p(ex')
    // always is about one execution's future, valid is about all executions
}

}
