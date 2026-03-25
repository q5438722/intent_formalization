use vstd::prelude::*;

fn main() {}

verus!{

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
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// ===== Target Function =====

pub proof fn leads_to_apply<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(p.leads_to(q)),
    ensures spec.entails(eventually(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies eventually(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, p);
        implies_apply::<T>(ex, spec, p.leads_to(q));
        leads_to_unfold::<T>(ex, p, q);
        execution_equality::<T>(ex, ex.suffix(0));
        implies_apply::<T>(ex, p, eventually(q));
    };
}

// ===== BOUNDARY TESTS =====

// Test 1: Missing spec.entails(p) — first precondition of leads_to_apply
// SHOULD FAIL
proof fn test_boundary_missing_entails_p<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        // MISSING: spec.entails(p)
    ensures spec.entails(eventually(q)),
{
    leads_to_apply(spec, p, q);
}

// Test 2: Missing spec.entails(p.leads_to(q)) — second precondition of leads_to_apply
// SHOULD FAIL
proof fn test_boundary_missing_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        // MISSING: spec.entails(p.leads_to(q))
    ensures spec.entails(eventually(q)),
{
    leads_to_apply(spec, p, q);
}

// Test 3: implies_apply without p.satisfied_by(ex) — missing second precondition
// SHOULD FAIL
proof fn test_boundary_implies_apply_missing_p<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        // MISSING: p.satisfied_by(ex)
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// Test 4: leads_to_unfold without the leads_to prerequisite — missing precondition
// SHOULD FAIL
proof fn test_boundary_leads_to_unfold_no_prereq<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    // MISSING: p.leads_to(q).satisfied_by(ex)
    ensures forall |i: nat| p.implies(eventually(q)).satisfied_by(#[trigger] ex.suffix(i)),
{
    leads_to_unfold(ex, p, q);
}

// Test 5: execution_equality without pointwise equality — missing precondition
// SHOULD FAIL
proof fn test_boundary_exec_equality_no_pointwise<T>(ex1: Execution<T>, ex2: Execution<T>)
    // MISSING: forall |i: nat| (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i)
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
}

}
