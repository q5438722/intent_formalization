// Boundary Tests: Violate preconditions of transform_leads_to_with_until
// Each test calls the function with one or more preconditions missing.
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

// ====== BOUNDARY TESTS ======

// Test 1: Missing precondition - spec.entails(p1.leads_to(q1))
// SHOULD FAIL: the first precondition of transform_leads_to_with_until is not established
proof fn test_missing_leads_to_precondition<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        // MISSING: spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        spec.entails(always(next)),
{
    // SHOULD FAIL: calling without the leads_to precondition
    transform_leads_to_with_until(spec, next, p1, q1, p2, q2);
}

// Test 2: Missing precondition - spec.entails(always(next))
// SHOULD FAIL: the always(next) precondition is not established
proof fn test_missing_always_next<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        // MISSING: spec.entails(always(next)),
{
    // SHOULD FAIL: calling without the always(next) precondition
    transform_leads_to_with_until(spec, next, p1, q1, p2, q2);
}

// Test 3: Missing precondition - inductive step for p2
// SHOULD FAIL: the inductive condition on p2 is not established
proof fn test_missing_inductive_step<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
        // MISSING: spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        spec.entails(always(next)),
{
    // SHOULD FAIL: calling without the inductive step precondition
    transform_leads_to_with_until(spec, next, p1, q1, p2, q2);
}

// Test 4: Weakened precondition - eventually(next) instead of always(next)
// SHOULD FAIL: eventually(next) does not satisfy the always(next) requirement
proof fn test_weakened_always_to_eventually<T>(
    spec: TempPred<T>, next: TempPred<T>,
    p1: TempPred<T>, q1: TempPred<T>,
    p2: TempPred<T>, q2: TempPred<T>
)
    requires
        spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.and(next).implies(later(p2).or(later(q2))))),
        spec.entails(eventually(next)),  // WEAKENED: eventually instead of always
{
    // SHOULD FAIL: eventually(next) is weaker than always(next)
    transform_leads_to_with_until(spec, next, p1, q1, p2, q2);
}

}
