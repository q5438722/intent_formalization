use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions from target file ==========

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

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn entails_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
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

#[verifier::external_body]
pub proof fn entails_and_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(q),
    ensures spec.entails(p.and(q)),
{
    unimplemented!()
}

pub proof fn simplify_predicate<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures p == p.and(q),
{
    assert forall |ex| #[trigger] p.satisfied_by(ex) implies p.and(q).satisfied_by(ex) by {
        entails_and_temp::<T>(p, p, q);
        entails_apply::<T>(ex, p, p.and(q));
    };
    temp_pred_equality::<T>(p, p.and(q));
}

// ========== BOUNDARY TESTS ==========
// These tests violate preconditions and should FAIL verification.

// SHOULD FAIL: Call simplify_predicate without the required p.entails(q)
proof fn test_simplify_no_entailment(p: TempPred<int>, q: TempPred<int>)
    ensures p == p.and(q),
{
    simplify_predicate(p, q);
}

// SHOULD FAIL: Call entails_apply without p.entails(q)
proof fn test_entails_apply_no_entailment(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    entails_apply(ex, p, q);
}

// SHOULD FAIL: Call entails_apply without p.satisfied_by(ex)
proof fn test_entails_apply_no_satisfaction(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures q.satisfied_by(ex),
{
    entails_apply(ex, p, q);
}

// SHOULD FAIL: Call temp_pred_equality with only one direction of entailment
proof fn test_equality_one_direction(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures p == q,
{
    temp_pred_equality(p, q);
}

// SHOULD FAIL: Call entails_and_temp with only one of the two required entailments
proof fn test_entails_and_temp_missing_q(spec: TempPred<int>, p: TempPred<int>, q: TempPred<int>)
    requires spec.entails(p),
    ensures spec.entails(p.and(q)),
{
    entails_and_temp(spec, p, q);
}

}
