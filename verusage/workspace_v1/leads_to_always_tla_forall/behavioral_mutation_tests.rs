use vstd::prelude::*;
use vstd::map_lib::*;

fn main() {}

verus! {

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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ===== Axioms (external_body proof functions) =====

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
    ensures eventually(p).satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

#[verifier::external_body]
pub proof fn leads_to_always_tla_forall<T, A>(
    spec_pred: TempPred<T>, p: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>, domain: Set<A>,
)
    requires
        forall |a: A| spec_pred.entails(p.leads_to(always(#[trigger] a_to_p(a)))),
        domain.finite(),
        domain.len() > 0,
        forall |a: A| #[trigger] domain.contains(a),
    ensures spec_pred.entails(p.leads_to(always(tla_forall(a_to_p)))),
{ unimplemented!() }

// ====================================================================
// BEHAVIORAL MUTATION TESTS — mutate expected relations; each SHOULD FAIL
// ====================================================================

// SHOULD FAIL
// Converse of modus ponens: from (p => q) and q, try to derive p
proof fn test_implies_converse(
    ex: Execution<int>, p: TempPred<int>, q: TempPred<int>,
)
    requires
        p.implies(q).satisfied_by(ex),
        q.satisfied_by(ex),
    ensures p.satisfied_by(ex),
{
    // Affirming the consequent is a fallacy — cannot derive p from (p => q) ∧ q
}

// SHOULD FAIL
// Mutate: eventually(p) should NOT imply always(p)
proof fn test_eventually_does_not_imply_always(
    ex: Execution<int>, p: TempPred<int>,
)
    requires eventually(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
    // "p holds at some point" does not mean "p holds at all points"
}

// SHOULD FAIL
// Strengthen conclusion: drop leads_to(p, ...) to get always(tla_forall) directly
proof fn test_leads_to_strengthened_to_always(
    spec_pred: TempPred<int>, p: TempPred<int>,
    a_to_p: spec_fn(int) -> TempPred<int>, domain: Set<int>,
)
    requires
        forall |a: int| spec_pred.entails(p.leads_to(always(#[trigger] a_to_p(a)))),
        domain.finite(),
        domain.len() > 0,
        forall |a: int| #[trigger] domain.contains(a),
    ensures spec_pred.entails(always(tla_forall(a_to_p))),
{
    leads_to_always_tla_forall(spec_pred, p, a_to_p, domain);
    // The lemma proves p.leads_to(always(tla_forall(a_to_p))),
    // NOT always(tla_forall(a_to_p)) directly
}

// SHOULD FAIL
// Reverse always_propagate: from always(p) at suffix, derive always(p) at original
proof fn test_always_propagate_reverse(
    ex: Execution<int>, p: TempPred<int>, i: nat,
)
    requires
        always(p).satisfied_by(ex.suffix(i)),
        i > 0,
    ensures always(p).satisfied_by(ex),
{
    // always(p) at a later suffix does not imply always(p) at the original execution
}

// SHOULD FAIL
// Mutate: swap p and q in leads_to — leads_to is not commutative
proof fn test_leads_to_swap_conclusion(
    spec_pred: TempPred<int>, p: TempPred<int>,
    a_to_p: spec_fn(int) -> TempPred<int>, domain: Set<int>,
)
    requires
        forall |a: int| spec_pred.entails(p.leads_to(always(#[trigger] a_to_p(a)))),
        domain.finite(),
        domain.len() > 0,
        forall |a: int| #[trigger] domain.contains(a),
    ensures spec_pred.entails(always(tla_forall(a_to_p)).leads_to(p)),
{
    leads_to_always_tla_forall(spec_pred, p, a_to_p, domain);
    // The lemma does not prove the reverse direction
}

}
