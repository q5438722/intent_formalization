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

// ---- Behavioral Mutation Tests ----
// Keep all preconditions of leads_to_always_enhance, but mutate the conclusion.

// Test 1: Reverse leads_to direction in conclusion
// The original proves p ↝ □q2; this claims □q2 ↝ p (backwards).
// SHOULD FAIL
proof fn mutation_reverse_conclusion<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        q1.and(inv).entails(q2),
    ensures
        spec.entails(always(q2).leads_to(p)),  // REVERSED direction
{
}

// Test 2: Reverse the entailment direction — assume q2 ⊨ q1∧inv instead of q1∧inv ⊨ q2
// This doesn't let us derive q2 from q1 at each suffix.
// SHOULD FAIL
proof fn mutation_swap_entailment<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        q2.entails(q1.and(inv)),  // REVERSED entailment
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

// Test 3: Remove spec guard from conclusion — claim valid(p ↝ □q2) for ALL executions
// The original only guarantees the conclusion under spec-satisfying executions.
// SHOULD FAIL
proof fn mutation_drop_spec_guard<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        q1.and(inv).entails(q2),
    ensures
        valid(p.leads_to(always(q2))),  // Global validity, no spec restriction
{
}

// Test 4: Replace inv with p in the entailment — p is not a global invariant
// p may only hold at the trigger point, not at every suffix.
// SHOULD FAIL
proof fn mutation_replace_inv_with_p<T>(
    spec: TempPred<T>, inv: TempPred<T>,
    p: TempPred<T>, q1: TempPred<T>, q2: TempPred<T>,
)
    requires
        spec.entails(always(inv)),
        spec.entails(p.leads_to(always(q1))),
        q1.and(p).entails(q2),  // MUTATED: p instead of inv
    ensures
        spec.entails(p.leads_to(always(q2))),
{
}

}
