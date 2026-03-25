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
// LOGICAL TESTS — unintended reasoning; each SHOULD FAIL verification
// ====================================================================

// SHOULD FAIL
// leads_to is NOT symmetric: p ~> q does NOT imply q ~> p
proof fn test_leads_to_not_symmetric(
    spec_pred: TempPred<int>, p: TempPred<int>, q: TempPred<int>,
)
    requires spec_pred.entails(p.leads_to(q)),
    ensures spec_pred.entails(q.leads_to(p)),
{
    // leads_to encodes liveness, not equivalence
}

// SHOULD FAIL
// Partial execution equality: agreeing at one index is NOT enough
proof fn test_execution_equality_partial(
    ex1: Execution<int>, ex2: Execution<int>,
)
    requires
        (ex1.nat_to_state)(0) == (ex2.nat_to_state)(0),
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
    // execution_equality requires pointwise equality at ALL indices
}

// SHOULD FAIL
// valid(p) does NOT imply valid(q) for arbitrary q
proof fn test_valid_does_not_transfer(
    p: TempPred<int>, q: TempPred<int>,
)
    requires valid(p),
    ensures valid(q),
{
    // valid(p) says nothing about unrelated q
}

// SHOULD FAIL
// ∀a. ◇(a_to_p(a)) does NOT imply ◇(∀a. a_to_p(a))
// (forall-eventually swap is invalid without finiteness + always)
proof fn test_forall_eventually_swap_invalid(
    ex: Execution<int>,
    a_to_p: spec_fn(int) -> TempPred<int>,
)
    requires
        forall |a: int| eventually(#[trigger] a_to_p(a)).satisfied_by(ex),
    ensures
        eventually(tla_forall(a_to_p)).satisfied_by(ex),
{
    // Each a_to_p(a) may hold at a DIFFERENT time;
    // there may be no single time where ALL hold simultaneously
}

// SHOULD FAIL
// entails is NOT symmetric: (spec ⊨ p) does NOT imply (p ⊨ spec)
proof fn test_entails_not_symmetric(
    spec_pred: TempPred<int>, p: TempPred<int>,
)
    requires spec_pred.entails(p),
    ensures p.entails(spec_pred),
{
    // entails is implication, not equivalence
}

}
