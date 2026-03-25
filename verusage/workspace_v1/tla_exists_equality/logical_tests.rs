use vstd::prelude::*;

fn main() {}

verus!{

// === Source definitions ===

pub type StatePred<T> = spec_fn(T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
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

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
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
{
    unimplemented!()
}

proof fn tla_exists_equality<T, A>(f: spec_fn(A, T) -> bool)
    ensures lift_state(|t| exists |a| #[trigger] f(a, t)) == tla_exists(|a| lift_state(|t| f(a, t))),
{
    let p = lift_state(|t| exists |a| #[trigger] f(a, t));
    let q = tla_exists(|a| lift_state(|t| f(a, t)));

    let partial_p = |t| exists |a| #[trigger] f(a, t);
    let partial_q = |a| lift_state(|t| f(a, t));
    assert forall |ex| p.satisfied_by(ex) implies q.satisfied_by(ex) by {
        assert(partial_p(ex.head()));
        assert(exists |a| #[trigger] f(a, ex.head()));
        let witness_a = choose |a| #[trigger] f(a, ex.head());
        assert(partial_q(witness_a).satisfied_by(ex));
    };

    temp_pred_equality::<T>(p, q);
}

// Helper for trigger in quantifier swap test
pub open spec fn head_equals(ex: Execution<int>, a: int) -> bool {
    ex.head() == a
}

pub open spec fn is_universal_head(a: int) -> bool {
    forall |ex: Execution<int>| #[trigger] head_equals(ex, a)
}

// === Logical Tests ===

// Test 1: Quantifier swap — forall-exists does NOT imply exists-forall
// SHOULD FAIL: swapping quantifier order produces a strictly stronger (false) claim
proof fn test_logical_quantifier_swap()
{
    let f = |a: int, t: int| a == t;
    tla_exists_equality::<int, int>(f);
    // From postcondition we know: lift_state(|t| ∃a. a==t) == tla_exists(|a| lift_state(|t| a==t))
    // valid(tla_exists(...)) means: ∀ex. ∃a. a == ex.head() — TRUE
    // But the STRONGER claim: ∃a. ∀ex. a == ex.head() — FALSE (no single a for all heads)
    assert(exists |a: int| #[trigger] is_universal_head(a)); // SHOULD FAIL
}

// Test 2: Wrong monotonicity — weaker predicate family does NOT entail stronger
// SHOULD FAIL: weakening f in tla_exists breaks the entailment direction
proof fn test_logical_wrong_monotonicity()
{
    let f_strong = |a: int, t: int| a == t && t > 0;
    let f_weak = |a: int, t: int| a == t;
    let p = tla_exists(|a: int| lift_state(|t: int| f_weak(a, t)));
    let q = tla_exists(|a: int| lift_state(|t: int| f_strong(a, t)));
    // p ≡ always true (for any head, pick a = head)
    // q ≡ head > 0 (pick a = head, but also need head > 0)
    // p does NOT entail q (head = -1: p true, q false)
    assert(p.entails(q)); // SHOULD FAIL
}

// Test 3: tla_exists over a universally false predicate family cannot be valid
// SHOULD FAIL: no witness a can satisfy a universally false predicate
proof fn test_logical_tla_exists_over_false()
{
    let p = tla_exists(|a: int| lift_state(|t: int| false));
    // p says: ∃a such that false — always false regardless of a
    // valid(p) = ∀ex. false = false
    assert(valid(p)); // SHOULD FAIL
}

}
