// Logical Tests: Properties NOT explicitly guaranteed by the specification
// Each test encodes a plausible but unsound logical inference.
// All tests SHOULD FAIL verification.

use vstd::prelude::*;

fn main() {}

verus!{

// ====== Type Definitions (from target) ======

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

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
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

pub open spec fn later<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.satisfied_by(ex.suffix(1)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ====== Helper Lemmas (from target) ======

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex)
{ unimplemented!() }

spec fn eventually_choose_witness<T>(ex: Execution<T>, p: TempPred<T>) -> nat
    recommends exists |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    let witness = choose |i| p.satisfied_by(#[trigger] ex.suffix(i));
    witness
}

#[verifier::external_body]
proof fn always_p_or_eventually_q<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
    ensures always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),
{ unimplemented!() }

// ====== Main Function Under Test ======

pub proof fn transform_leads_to_with_until<T>(spec: TempPred<T>, next: TempPred<T>, p1: TempPred<T>, q1: TempPred<T>, p2: TempPred<T>, q2: TempPred<T>)
    requires
        spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        spec.entails(always(next)),
    ensures
        spec.entails(p1.and(p2).leads_to((q1.and(p2)).or(q2))),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p1.and(p2).leads_to((q1.and(p2)).or(q2)).satisfied_by(ex) by {
        assert forall |i| #[trigger] p1.and(p2).satisfied_by(ex.suffix(i))
        implies eventually((q1.and(p2)).or(q2)).satisfied_by(ex.suffix(i)) by {
            implies_apply::<T>(ex, spec, always(next));
            implies_apply::<T>(ex, spec, always(p2.and(next).implies(later(p2).or(later(q2)))));
            always_p_or_eventually_q::<T>(ex, next, p2, q2);
            implies_apply::<T>(ex.suffix(i), p2, always(p2).or(eventually(q2)));
            implies_apply::<T>(ex, spec, p1.leads_to(q1));
            implies_apply::<T>(ex.suffix(i), p1, eventually(q1));
            if eventually(q2).satisfied_by(ex.suffix(i)) {
                let witness_idx = eventually_choose_witness::<T>(ex.suffix(i), q2);
                eventually_proved_by_witness::<T>(ex.suffix(i), (q1.and(p2)).or(q2), witness_idx);
            } else {
                let witness_idx = eventually_choose_witness::<T>(ex.suffix(i), q1);
                always_unfold::<T>(ex.suffix(i), p2);
                assert(p2.satisfied_by(ex.suffix(i).suffix(witness_idx)));
                assert(q1.and(p2).satisfied_by(ex.suffix(i).suffix(witness_idx)));
                eventually_proved_by_witness::<T>(ex.suffix(i), (q1.and(p2)).or(q2), witness_idx);
            }
        }
    }
}

// ====== LOGICAL TESTS ======

// Test 1: leads_to does NOT preserve conjunctions without inductive conditions
// Assumes only spec ⊨ p1 ~> q1, and tries to derive spec ⊨ (p1∧p2) ~> (q1∧p2)
// SHOULD FAIL: p2 may not be preserved while waiting for q1, without an inductive condition
proof fn test_leads_to_preserves_conjunction<T>(
    spec: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
    ensures
        spec.entails(p1.and(p2).leads_to(q1.and(p2))),  // NOT guaranteed without inductive step
{
    // SHOULD FAIL: leads_to does not preserve conjunctions automatically
}

// Test 2: always(next) alone (without inductive step) is insufficient
// Assumes spec ⊨ p1 ~> q1 and spec ⊨ □next, tries to derive the full conclusion
// SHOULD FAIL: the inductive condition on p2 is essential
proof fn test_always_next_alone_insufficient<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
        spec.entails(always(next)),
        // MISSING: spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
    ensures
        spec.entails(p1.and(p2).leads_to((q1.and(p2)).or(q2))),
{
    // SHOULD FAIL: without the inductive condition, p2's behavior is unknown
}

// Test 3: Swapped preconditions do NOT yield the original conclusion
// Uses p2.leads_to(q2) and inductive step on p1, but asserts original form
// SHOULD FAIL: different instantiation gives a structurally different result
proof fn test_swapped_roles_invalid<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p2.leads_to(q2)),  // SWAPPED: p2~>q2 instead of p1~>q1
        spec.entails(always(p1.and(next).implies(later(p1).or(later(q1))))),  // SWAPPED
        spec.entails(always(next)),
    ensures
        // Asserting the ORIGINAL conclusion (with p1, q1 in their original roles)
        spec.entails(p1.and(p2).leads_to((q1.and(p2)).or(q2))),
{
    // SHOULD FAIL: the swapped preconditions yield a different conclusion
    // Correct conclusion would be: p2.and(p1).leads_to((q2.and(p1)).or(q1))
}

// Test 4: Strengthening eventually to always in the conclusion
// Tries to derive □((q1∧p2)∨q2) instead of ◇((q1∧p2)∨q2)
// SHOULD FAIL: leads_to gives eventually, not always
proof fn test_strengthen_eventually_to_always<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        spec.entails(always(next)),
    ensures
        // MUTATED: always(p1∧p2 → □((q1∧p2)∨q2)) instead of leads_to
        spec.entails(always(p1.and(p2).implies(always((q1.and(p2)).or(q2))))),
{
    // SHOULD FAIL: leads_to gives eventually, not always
    transform_leads_to_with_until(spec, next, p1, q1, p2, q2);
}

}
