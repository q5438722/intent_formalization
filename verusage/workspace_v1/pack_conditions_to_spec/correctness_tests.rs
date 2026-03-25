use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions (from source) =====

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

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ===== Helper lemmas (trusted axioms) =====

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
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

// ===== Target function =====

pub proof fn pack_conditions_to_spec<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
    ensures spec.and(always(c)).entails(p.leads_to(q)),
{
    assert forall |ex| #[trigger] spec.and(always(c)).satisfied_by(ex) implies p.leads_to(q).satisfied_by(ex) by {
        implies_apply(ex, spec, p.and(c).leads_to(q));
        leads_to_unfold(ex, p.and(c), q);
        assert forall |i| #[trigger] p.satisfied_by(ex.suffix(i)) implies eventually(q).satisfied_by(ex.suffix(i)) by {
            always_propagate_forwards(ex, c, i);
            implies_apply(ex.suffix(i), p.and(c), eventually(q));
        }
    }
}

// =============================================================================
// BOUNDARY TESTS — Violate preconditions to check input rejection
// =============================================================================

// Boundary Test 1: No precondition at all.
// SHOULD FAIL
proof fn boundary_no_precondition<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    ensures spec.and(always(c)).entails(p.leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// Boundary Test 2: Precondition has p and q swapped.
// SHOULD FAIL
proof fn boundary_swapped_pq_precondition<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(q.and(c).leads_to(p)),
    ensures spec.and(always(c)).entails(p.leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// Boundary Test 3: Unrelated precondition (spec entails □p, irrelevant to leads_to).
// SHOULD FAIL
proof fn boundary_unrelated_precondition<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.and(always(c)).entails(p.leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// =============================================================================
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs to check rejection
// =============================================================================

// Mutation Test 1: Drop always(c) from conclusion — claims spec alone suffices.
// SHOULD FAIL
proof fn mutation_drop_always_c<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
    ensures spec.entails(p.leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// Mutation Test 2: Swap p and q in conclusion — wrong direction.
// SHOULD FAIL
proof fn mutation_swap_pq_conclusion<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
    ensures spec.and(always(c)).entails(q.leads_to(p)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// Mutation Test 3: Strengthen conclusion to □q.
// SHOULD FAIL
proof fn mutation_strengthen_to_always_q<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
    ensures spec.and(always(c)).entails(always(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
}

// =============================================================================
// LOGICAL TESTS — Non-guaranteed properties / unintended reasoning
// =============================================================================

// Logical Test 1: Derive pointwise p⇒q from temporal p ~> q.
// leads_to is temporal (eventually), not pointwise (at every step).
// SHOULD FAIL
proof fn logical_implies_from_leads_to<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
    assert(spec.and(always(c)).entails(p.implies(q)));
}

// Logical Test 2: Converse unpacking — reverse pack_conditions_to_spec direction.
// Knowing p ~> q under □c does NOT recover (p∧c) ~> q under spec alone.
// SHOULD FAIL
proof fn logical_converse_unpacking<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.and(always(c)).entails(p.leads_to(q)),
    ensures spec.entails(p.and(c).leads_to(q)),
{
}

// Logical Test 3: Claim □p ⊨ □q — structural global assumption.
// leads_to does not establish that globally-p implies globally-q.
// SHOULD FAIL
proof fn logical_always_p_implies_always_q<T>(spec: TempPred<T>, c: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.and(c).leads_to(q)),
{
    pack_conditions_to_spec(spec, c, p, q);
    assert(always(p).entails(always(q)));
}

}
