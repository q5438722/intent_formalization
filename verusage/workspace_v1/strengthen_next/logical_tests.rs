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

// ===== LOGICAL TESTS =====

// Test 1: spec.entails(p) does NOT imply valid(p)
// entails is relative to spec; valid means universally true
// SHOULD FAIL
proof fn test_logical_entails_does_not_imply_valid()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);

    // spec.entails(p) is true: head==0 implies head>=0
    assume(spec.entails(p));

    // But valid(p) requires ALL executions to have head>=0, not just those satisfying spec
    assert(valid(p));
}

// Test 2: spec.entails(p) does NOT imply spec.entails(always(p))
// Temporal predicate holding on an execution doesn't mean it holds on all suffixes
// SHOULD FAIL
proof fn test_logical_entails_does_not_lift_to_always()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);

    // spec.entails(p): for all ex, head==0 ==> head>=0 (true)
    assume(spec.entails(p));

    // spec.entails(always(p)): for all ex, head==0 ==> forall i, suffix(i).head >= 0
    // This is NOT guaranteed — p on an execution doesn't mean p on all suffixes
    assert(spec.entails(always(p)));
}

// Test 3: Cross-spec entailment is invalid
// spec1.entails(p) and spec2.entails(q) does NOT imply spec1.entails(q)
// SHOULD FAIL
proof fn test_logical_cross_spec_entailment()
{
    let spec1: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let spec2: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 1);
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 1);

    // Both assumptions are consistent and true
    assume(spec1.entails(p));  // head==0 ==> head>=0
    assume(spec2.entails(q));  // head==1 ==> head==1

    // Invalid: spec1 entailing p and spec2 entailing q does NOT mean spec1 entails q
    // spec1.entails(q) means: head==0 ==> head==1, which is FALSE
    assert(spec1.entails(q));
}

// Test 4: valid(p) for a non-universal predicate
// Not all executions satisfy p, so valid(p) should be unprovable
// SHOULD FAIL
proof fn test_logical_false_valid()
{
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() > 0);

    // Not all executions start with a positive value (e.g., head = -1)
    assert(valid(p));
}

// Test 5: Entails is NOT symmetric — p.entails(q) does NOT imply q.entails(p)
// SHOULD FAIL
proof fn test_logical_entails_not_symmetric()
{
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);

    // p.entails(q) is TRUE: head==0 ==> head>=0
    assume(p.entails(q));

    // q.entails(p) is FALSE: head>=0 does NOT imply head==0 (e.g., head=1)
    assert(q.entails(p));
}

}
