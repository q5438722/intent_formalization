use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions (from target file) =====

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

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ===== Helper lemmas (from target file) =====

#[verifier::external_body]
proof fn stable_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires stable(p).satisfied_by(ex),
    ensures p.satisfied_by(ex) ==> forall |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier::spinoff_prover]
proof fn leads_to_unfold<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.leads_to(q).satisfied_by(ex),
    ensures forall |i: nat| p.implies(eventually(q)).satisfied_by(#[trigger] ex.suffix(i)),
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

// ===== Target function under test =====

pub proof fn unpack_conditions_from_spec<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(spec)),
        spec.and(c).entails(p.leads_to(q)),
    ensures spec.entails(p.and(c).leads_to(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.and(c).leads_to(q).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.and(c).satisfied_by(ex.suffix(i)) implies eventually(q).satisfied_by(ex.suffix(i)) by {
            stable_unfold::<T>(ex, spec);
            implies_apply::<T>(ex.suffix(i), spec.and(c), p.leads_to(q));
            leads_to_unfold::<T>(ex.suffix(i), p, q);
            execution_equality::<T>(ex.suffix(i), ex.suffix(i).suffix(0));
            implies_apply::<T>(ex.suffix(i), p, eventually(q));
        };
    };
}


// ========================================================================
// BEHAVIORAL MUTATION TESTS: Valid inputs, mutated/incorrect outputs
// ========================================================================

// MUTATION TEST 1: Drop c from the leads_to antecedent
// The proved result is spec.entails(p.and(c).leads_to(q)).
// Here we assert the stronger spec.entails(p.leads_to(q)) — c is dropped entirely.
// This is incorrect: without c in the antecedent, p alone need not lead to q.
// SHOULD FAIL
proof fn mutation_test_drop_c_from_antecedent<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(spec)),
        spec.and(c).entails(p.leads_to(q)),
{
    unpack_conditions_from_spec(spec, c, p, q);
    assert(spec.entails(p.leads_to(q)));
}

// MUTATION TEST 2: Swap p and q in the leads_to conclusion
// The proved result is spec.entails(p.and(c).leads_to(q)).
// Here we assert spec.entails(q.and(c).leads_to(p)) — swapping antecedent and consequent.
// leads_to is not symmetric; q leading to p is a completely different claim.
// SHOULD FAIL
proof fn mutation_test_swap_p_q<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(spec)),
        spec.and(c).entails(p.leads_to(q)),
{
    unpack_conditions_from_spec(spec, c, p, q);
    assert(spec.entails(q.and(c).leads_to(p)));
}

// MUTATION TEST 3: Strengthen the consequent by conjoining c
// The proved result is spec.entails(p.and(c).leads_to(q)).
// Here we assert spec.entails(p.and(c).leads_to(q.and(c))) — the consequent is strengthened.
// Nothing guarantees c still holds when q is eventually reached.
// SHOULD FAIL
proof fn mutation_test_strengthen_consequent<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(spec)),
        spec.and(c).entails(p.leads_to(q)),
{
    unpack_conditions_from_spec(spec, c, p, q);
    assert(spec.entails(p.and(c).leads_to(q.and(c))));
}


}
