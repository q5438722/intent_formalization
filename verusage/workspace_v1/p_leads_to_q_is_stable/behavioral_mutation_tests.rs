use vstd::prelude::*;

fn main() {}

verus! {

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

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
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

#[verifier::external_body]
pub proof fn always_p_is_stable<T>(p: TempPred<T>)
    ensures valid(stable(always(p))),
{
    unimplemented!()
}

pub proof fn p_leads_to_q_is_stable<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(stable(p.leads_to(q))),
{
    always_p_is_stable(p.implies(eventually(q)));
}

// ===== Behavioral Mutation Tests =====

// Test 1: Swap p and q — SHOULD FAIL
// Calling p_leads_to_q_is_stable(p, q) gives stable(p ~> q).
// This should NOT give us stable(q ~> p) — the relation is not symmetric.
proof fn test_swap_leads_to_direction<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(stable(q.leads_to(p))), // SHOULD FAIL
{
    p_leads_to_q_is_stable(p, q);
    // ensures gives valid(stable(p.leads_to(q))), NOT valid(stable(q.leads_to(p)))
}

// Test 2: Strengthen stable to unconditional always — SHOULD FAIL
// stable(X) means "if X holds, then always(X) holds"
// always(X) means "X holds at every point" — strictly stronger
proof fn test_strengthen_stable_to_always<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(always(p.leads_to(q))), // SHOULD FAIL
{
    p_leads_to_q_is_stable(p, q);
    // stable(X) ≠ always(X); stable only says "if X then always X"
}

// Test 3: Assert leads_to holds at a specific execution — SHOULD FAIL
// Stability does not assert that a property holds at any particular execution.
proof fn test_leads_to_at_specific_execution<T>(p: TempPred<T>, q: TempPred<T>, ex: Execution<T>)
    ensures p.leads_to(q).satisfied_by(ex), // SHOULD FAIL
{
    p_leads_to_q_is_stable(p, q);
    // stable(p ~> q) only says: IF p~>q holds at ex, THEN always(p~>q) holds at ex
    // It does NOT assert that p~>q holds at ex
}

}
