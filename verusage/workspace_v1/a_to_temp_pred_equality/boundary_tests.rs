use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions from target file ===

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
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

proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{
    assert forall |a: A| #[trigger] p(a) == q(a) by {
        temp_pred_equality::<T>(p(a), q(a));
    };
    assert(p =~= q);
}

// === BOUNDARY TESTS ===

// Test 1: Call temp_pred_equality with contradictory predicates (always-true vs always-false).
// p.entails(q) requires forall |ex| true ==> false, which is false.
// SHOULD FAIL
proof fn test_boundary_contradictory_predicates()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    temp_pred_equality::<int>(p, q);
}

// Test 2: Call temp_pred_equality with only one direction of entailment.
// p.entails(q) holds, but q.entails(p) is explicitly false.
// SHOULD FAIL
proof fn test_boundary_one_direction_only(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        !(q.entails(p)),
{
    temp_pred_equality::<int>(p, q);
}

// Test 3: Call a_to_temp_pred_equality with condition holding only at a single point,
// not universally over all inputs.
// SHOULD FAIL
proof fn test_boundary_a_to_partial_condition(
    p: spec_fn(int) -> TempPred<int>,
    q: spec_fn(int) -> TempPred<int>,
)
    requires
        p(0).entails(q(0)) && q(0).entails(p(0)),
{
    a_to_temp_pred_equality::<int, int>(p, q);
}

}
