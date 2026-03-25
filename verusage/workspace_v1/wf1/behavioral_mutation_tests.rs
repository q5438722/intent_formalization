use vstd::prelude::*;

fn main() {}

verus! {

// ========== Type Definitions ==========

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
        Execution { nat_to_state: |i: nat| (self.nat_to_state)(i + pos) }
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

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn lift_action<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| action_pred(ex.head(), ex.head_next()))
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

pub open spec fn enabled<T>(action_pred: ActionPred<T>) -> StatePred<T> {
    |s: T| exists |s_prime: T| #[trigger] action_pred(s, s_prime)
}

pub open spec fn weak_fairness<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    always(lift_state(enabled(action_pred))).leads_to(lift_action(action_pred))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ========== Proof Functions (trusted axioms for testing) ==========

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{}

#[verifier::external_body]
proof fn implies_apply_with_always<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures always(q).satisfied_by(ex),
{}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{}

#[verifier::external_body]
pub proof fn wf1<T>(spec: TempPred<T>, next: ActionPred<T>, forward: ActionPred<T>, p: StatePred<T>, q: StatePred<T>)
    requires
        forall |s, s_prime: T| p(s) && #[trigger] next(s, s_prime) ==> p(s_prime) || q(s_prime),
        forall |s, s_prime: T| p(s) && #[trigger] next(s, s_prime) && forward(s, s_prime) ==> q(s_prime),
        forall |s: T| #[trigger] p(s) ==> enabled(forward)(s),
        spec.entails(always(lift_action(next))),
        spec.entails(weak_fairness(forward)),
    ensures spec.entails(lift_state(p).leads_to(lift_state(q))),
{}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Knowing p leads_to q does NOT imply p leads_to r (wrong target).
// Mutates the target predicate of leads_to.
// SHOULD FAIL
proof fn test_mutation_wrong_target(
    ex: Execution<int>,
    p: TempPred<int>,
    q: TempPred<int>,
    r: TempPred<int>,
)
    requires
        p.leads_to(q).satisfied_by(ex),
{
    assert(p.leads_to(r).satisfied_by(ex));
}

// Test 2: Knowing p leads_to q does NOT imply q leads_to p (reversed).
// Leads-to is not symmetric.
// SHOULD FAIL
proof fn test_mutation_reversed_leads_to(
    ex: Execution<int>,
    p: TempPred<int>,
    q: TempPred<int>,
)
    requires
        p.leads_to(q).satisfied_by(ex),
{
    assert(q.leads_to(p).satisfied_by(ex));
}

// Test 3: Knowing spec entails p.leads_to(q) does NOT imply spec entails always(q).
// leads_to is conditional; always(q) is unconditional and much stronger.
// SHOULD FAIL
proof fn test_mutation_strengthen_to_always(
    spec_pred: TempPred<int>,
    p: TempPred<int>,
    q: TempPred<int>,
)
    requires
        spec_pred.entails(p.leads_to(q)),
{
    assert(spec_pred.entails(always(q)));
}

// Test 4: After implies_apply gives us q, asserting arbitrary r should fail.
// Mutates the conclusion of implies_apply.
// SHOULD FAIL
proof fn test_mutation_implies_wrong_conclusion(
    ex: Execution<int>,
    p: TempPred<int>,
    q: TempPred<int>,
    r: TempPred<int>,
)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
{
    implies_apply(ex, p, q);
    // After implies_apply, we have q.satisfied_by(ex).
    // But r is completely arbitrary — should not be derivable.
    assert(r.satisfied_by(ex));
}

}
