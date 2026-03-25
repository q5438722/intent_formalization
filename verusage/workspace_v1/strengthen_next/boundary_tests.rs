use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions (from source) =====

pub type StatePred<T> = spec_fn(T) -> bool;
pub type ActionPred<T> = spec_fn(T, T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }
    pub open spec fn head_next(self) -> T {
        (self.nat_to_state)(1)
    }
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
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn lift_action<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| action_pred(ex.head(), ex.head_next()))
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
pub proof fn always_and_equality<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.and(q)) == always(p).and(always(q)),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn entails_and_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(q),
    ensures spec.entails(p.and(q)),
{ unimplemented!() }

pub proof fn strengthen_next<T>(spec: TempPred<T>, next: ActionPred<T>, inv: StatePred<T>, next_and_inv: ActionPred<T>)
    requires
        spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))),
        lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)),
    ensures spec.entails(always(lift_action(next_and_inv))),
{
    entails_and_temp::<T>(spec, always(lift_action(next)), always(lift_state(inv)));
    always_and_equality::<T>(lift_action(next), lift_state(inv));
    temp_pred_equality::<T>(lift_action(next_and_inv), lift_action(next).and(lift_state(inv)));
}

// ===== BOUNDARY TESTS =====

// Test 1: Missing precondition — spec does NOT entail always(lift_action(next))
// SHOULD FAIL
proof fn test_boundary_missing_always_next()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    let next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0;

    // Provide all preconditions EXCEPT spec.entails(always(lift_action(next)))
    assume(spec.entails(always(lift_state(inv))));
    assume(lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)));

    strengthen_next(spec, next, inv, next_and_inv);
}

// Test 2: Missing precondition — spec does NOT entail always(lift_state(inv))
// SHOULD FAIL
proof fn test_boundary_missing_always_inv()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    let next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0;

    // Provide all preconditions EXCEPT spec.entails(always(lift_state(inv)))
    assume(spec.entails(always(lift_action(next))));
    assume(lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)));

    strengthen_next(spec, next, inv, next_and_inv);
}

// Test 3: Missing forward equivalence — next_and_inv does NOT entail next ∧ inv
// Use a WEAKER next_and_inv so forward direction is genuinely unprovable
// SHOULD FAIL
proof fn test_boundary_missing_forward_equiv()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    // Weaker: only checks inv, not next — forward direction is FALSE
    let weak_next_and_inv: ActionPred<int> = |s1: int, s2: int| s1 >= 0;

    // Provide all preconditions EXCEPT forward equiv
    assume(spec.entails(always(lift_action(next))));
    assume(spec.entails(always(lift_state(inv))));
    // Backward: (s2==s1+1 && s1>=0) => s1>=0 — true, so this assume is consistent
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(weak_next_and_inv)));
    // Missing: lift_action(weak_next_and_inv).entails(lift_action(next).and(lift_state(inv)))
    // s1>=0 does NOT imply (s2==s1+1 && s1>=0) — genuinely unprovable

    strengthen_next(spec, next, inv, weak_next_and_inv);
}

// Test 4: Missing backward equivalence — next ∧ inv does NOT entail next_and_inv
// Use a STRONGER next_and_inv so backward direction is genuinely unprovable
// SHOULD FAIL
proof fn test_boundary_missing_backward_equiv()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    // Stronger: adds an extra constraint (s2 <= 100) — backward direction is FALSE
    let strong_next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0 && s2 <= 100;

    // Provide all preconditions EXCEPT backward equiv
    assume(spec.entails(always(lift_action(next))));
    assume(spec.entails(always(lift_state(inv))));
    // Forward: (s2==s1+1 && s1>=0 && s2<=100) => (s2==s1+1 && s1>=0) — true
    assume(lift_action(strong_next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    // Missing: lift_action(next).and(lift_state(inv)).entails(lift_action(strong_next_and_inv))
    // (s2==s1+1 && s1>=0) does NOT imply (s2<=100) — genuinely unprovable

    strengthen_next(spec, next, inv, strong_next_and_inv);
}

// Test 5: temp_pred_equality with only one direction of entailment
// SHOULD FAIL
proof fn test_boundary_equality_one_direction()
{
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);

    // Only p.entails(q) holds (0 >= 0), NOT q.entails(p) (1 >= 0 but 1 != 0)
    assume(p.entails(q));
    // Missing: q.entails(p)

    temp_pred_equality(p, q);
}

// Test 6: entails_and_temp with only one of the two entailments
// Use q that spec genuinely does NOT entail
// SHOULD FAIL
proof fn test_boundary_entails_and_missing_second()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);
    // q requires head == 5, which is NOT entailed by spec (head == 0)
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 5);

    // Only provide spec.entails(p), NOT spec.entails(q)
    assume(spec.entails(p));
    // Missing: spec.entails(q) — head==0 does NOT imply head==5

    entails_and_temp(spec, p, q);
}

}
