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

// ===== Logical Tests =====

// Test 1: leads_to implies eventually(q) without assuming p — SHOULD FAIL
// p ~> q says "whenever p holds, eventually q holds"
// This does NOT guarantee q eventually holds without p being assumed.
proof fn test_leads_to_implies_eventual_q<T>(p: TempPred<T>, q: TempPred<T>, ex: Execution<T>)
    requires p.leads_to(q).satisfied_by(ex),
    ensures eventually(q).satisfied_by(ex), // SHOULD FAIL
{
    // p ~> q = always(p => eventually(q))
    // Without knowing p holds, we cannot conclude eventually(q)
}

// Test 2: Misuse always_p_is_stable to prove stability of non-always predicate — SHOULD FAIL
// always_p_is_stable gives stable(always(p)), but calling it with p
// should NOT yield stable(p) itself.
proof fn test_misuse_always_stable_for_plain_pred<T>(p: TempPred<T>)
    ensures valid(stable(p)), // SHOULD FAIL
{
    always_p_is_stable(p);
    // This gives valid(stable(always(p))), which is NOT valid(stable(p))
}

// Test 3: Transitivity of leads_to — SHOULD FAIL
// The spec does not prove that (p ~> q) ∧ (q ~> r) => (p ~> r).
// Transitivity is a separate theorem requiring its own proof.
proof fn test_leads_to_transitivity<T>(
    p: TempPred<T>, q: TempPred<T>, r: TempPred<T>, ex: Execution<T>
)
    requires
        p.leads_to(q).satisfied_by(ex),
        q.leads_to(r).satisfied_by(ex),
    ensures p.leads_to(r).satisfied_by(ex), // SHOULD FAIL
{
    // No available lemma proves transitivity of leads_to
}

}
