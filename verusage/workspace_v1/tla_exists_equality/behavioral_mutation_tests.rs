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

// === Behavioral Mutation Tests ===

// Test 1: Negate the equality result from tla_exists_equality
// SHOULD FAIL: the ensures clause guarantees equality; asserting inequality contradicts it
proof fn test_mutation_negate_equality()
{
    let f = |a: int, t: int| a == t;
    tla_exists_equality::<int, int>(f);
    let p = lift_state(|t: int| exists |a: int| #[trigger] f(a, t));
    let q = tla_exists(|a: int| lift_state(|t: int| f(a, t)));
    assert(p != q); // SHOULD FAIL: postcondition says p == q
}

// Test 2: Replace exists with forall on the LHS — mutating the quantifier
// SHOULD FAIL: forall is strictly stronger than exists; the equality does not hold
proof fn test_mutation_forall_replaces_exists()
{
    let f = |a: int, t: int| a == t;
    // LHS: lift_state(|t| forall |a| a == t) ≡ lift_state(|t| false) [no t equals all ints]
    // RHS: tla_exists(|a| lift_state(|t| a == t)) ≡ always true [pick a = head]
    let p = lift_state(|t: int| forall |a: int| #[trigger] f(a, t));
    let q = tla_exists(|a: int| lift_state(|t: int| f(a, t)));
    assert(p == q); // SHOULD FAIL: false ≠ true
}

// Test 3: Use different functions on each side of the equality
// SHOULD FAIL: different predicates produce different temporal properties
proof fn test_mutation_different_function()
{
    let f1 = |a: int, t: int| a == t && t > 5;
    let f2 = |a: int, t: int| a == t && t < 0;
    // p ≡ lift_state(|t| t > 5): true when head > 5
    // q ≡ lift_state(|t| t < 0): true when head < 0
    let p = lift_state(|t: int| exists |a: int| #[trigger] f1(a, t));
    let q = tla_exists(|a: int| lift_state(|t: int| f2(a, t)));
    assert(p == q); // SHOULD FAIL: head > 5 ≠ head < 0
}

}
