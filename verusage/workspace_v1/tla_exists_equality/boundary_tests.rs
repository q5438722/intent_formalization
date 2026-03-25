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

// === Boundary Tests ===

// Test 1: One-direction entailment only (false → true holds vacuously, but true → false does not)
// SHOULD FAIL: q.entails(p) precondition of temp_pred_equality is violated
proof fn test_boundary_one_direction_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| false);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    // p.entails(q) holds vacuously (false ==> anything is true)
    // q.entails(p) does NOT hold (true ==> false is false)
    temp_pred_equality::<int>(p, q);
}

// Test 2: No entailment in either direction (disjoint predicates)
// SHOULD FAIL: both preconditions of temp_pred_equality are violated
proof fn test_boundary_no_entailment()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| ex.head() > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| ex.head() < 0);
    // p does not entail q (head=1: p true, q false)
    // q does not entail p (head=-1: q true, p false)
    temp_pred_equality::<int>(p, q);
}

// Test 3: Off-by-one boundary — strict vs non-strict inequality
// SHOULD FAIL: p.entails(q) violated at boundary value (head == 0)
proof fn test_boundary_strict_vs_nonstrict()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| ex.head() >= 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| ex.head() > 0);
    // q.entails(p) holds (head > 0 ⟹ head ≥ 0)
    // p.entails(q) does NOT hold (head = 0: p true, q false)
    temp_pred_equality::<int>(p, q);
}

}
