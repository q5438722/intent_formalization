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

// ===== Boundary Tests =====

// Test 1: Arbitrary predicate is stable — SHOULD FAIL
// The spec only proves always(p) is stable, not arbitrary p.
// An arbitrary predicate can hold at one point and cease later.
proof fn test_arbitrary_predicate_is_stable<T>(p: TempPred<T>)
    ensures valid(stable(p)), // SHOULD FAIL
{
    always_p_is_stable(p);
    // always_p_is_stable gives valid(stable(always(p))), NOT valid(stable(p))
}

// Test 2: eventually(p) is stable — SHOULD FAIL
// eventually(p) can hold at a state without holding at all future states,
// so it is not necessarily stable.
proof fn test_eventually_is_stable<T>(p: TempPred<T>)
    ensures valid(stable(eventually(p))), // SHOULD FAIL
{
    always_p_is_stable(p);
    // stable(always(p)) does not imply stable(eventually(p))
}

// Test 3: leads_to is unconditionally valid — SHOULD FAIL
// The spec only proves leads_to is *stable*, not that it holds for all executions.
proof fn test_leads_to_is_unconditionally_valid<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(p.leads_to(q)), // SHOULD FAIL
{
    p_leads_to_q_is_stable(p, q);
    // stability ≠ validity: stable(X) says "if X, then always X", not "X"
}

}
