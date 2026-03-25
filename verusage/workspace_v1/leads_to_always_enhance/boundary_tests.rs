use vstd::prelude::*;

fn main() {}

verus!{

// ---- Definitions (from target file) ----

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

// ---- Boundary Tests ----
// Each test drops or weakens a precondition of leads_to_always_enhance.
// The original requires:
//   (1) spec.entails(always(inv))
//   (2) spec.entails(p.leads_to(always(q1)))
//   (3) q1.and(inv).entails(q2)
// Conclusion: spec.entails(p.leads_to(always(q2)))

// Test 1: Drop precondition (1) — invariant not assumed
// Without always(inv), we cannot derive q2 from q1 at arbitrary suffixes.
// SHOULD FAIL
proof fn boundary_drop_invariant<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        // spec.entails(always(inv)), // DROPPED
        spec.entails(p.leads_to(always(q1))),
        q1.and(inv).entails(q2),
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

// Test 2: Drop precondition (2) — leads_to not assumed
// Without knowing p leads to always(q1), conclusion is unsupported.
// SHOULD FAIL
proof fn boundary_drop_leads_to<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        // spec.entails(p.leads_to(always(q1))), // DROPPED
        q1.and(inv).entails(q2),
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

// Test 3: Drop precondition (3) — entailment q1∧inv ⊨ q2 not assumed
// Without connecting q1∧inv to q2, conclusion about q2 is unsupported.
// SHOULD FAIL
proof fn boundary_drop_entailment<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        // q1.and(inv).entails(q2), // DROPPED
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

// Test 4: Weaken precondition (2) — leads_to(q1) instead of leads_to(always(q1))
// Momentary q1 is not enough to get always(q2).
// SHOULD FAIL
proof fn boundary_weaken_leads_to_no_always<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(q1)),  // q1, NOT always(q1)
        q1.and(inv).entails(q2),
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

}
