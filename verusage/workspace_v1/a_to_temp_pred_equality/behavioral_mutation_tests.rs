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

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: Valid mutual entailment, but assert strengthened conclusion valid(p).
// The postcondition only guarantees p == q, not that p is valid (holds for all executions).
// SHOULD FAIL
proof fn test_mutation_strengthened_to_valid(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    temp_pred_equality::<int>(p, q);
    assert(valid(p));
}

// Test 2: Valid mutual entailment between p and q, but assert equality with unrelated r.
// The postcondition only establishes p == q, not p == r.
// SHOULD FAIL
proof fn test_mutation_equality_with_unrelated(p: TempPred<int>, q: TempPred<int>, r: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    temp_pred_equality::<int>(p, q);
    assert(p == r);
}

// Test 3: Valid mutual entailment, but assert p entails an arbitrary unrelated predicate r.
// Equality of p and q does not extend entailment to arbitrary predicates.
// SHOULD FAIL
proof fn test_mutation_entails_arbitrary(p: TempPred<int>, q: TempPred<int>, r: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    temp_pred_equality::<int>(p, q);
    assert(p.entails(r));
}

}
