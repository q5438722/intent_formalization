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

// ========== BEHAVIORAL MUTATION TESTS ==========
// These tests mutate the expected output/relation and should FAIL verification.

// SHOULD FAIL: Mutated conclusion — p == q instead of p == p.and(q)
proof fn test_simplify_mutated_to_equality(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures p == q,
{
    simplify_predicate(p, q);
}

// SHOULD FAIL: Mutated direction — q == q.and(p) instead of p == p.and(q)
proof fn test_simplify_swapped_subjects(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures q == q.and(p),
{
    simplify_predicate(p, q);
}

// SHOULD FAIL: Reversed entails_apply — from p.entails(q) and q.satisfied_by(ex), derive p.satisfied_by(ex)
proof fn test_entails_apply_reversed(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.satisfied_by(ex),
    ensures p.satisfied_by(ex),
{
    entails_apply(ex, p, q);
}

// SHOULD FAIL: Mutated postcondition of temp_pred_equality — instead of p == q, conclude p.entails(q.and(q))
// which should follow, but try to additionally conclude q == p.and(q), which requires q.entails(p)
proof fn test_equality_mutated_extra(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
    ensures q == p.and(q),
{
    // temp_pred_equality gives p == q, but not q == p.and(q) directly
    temp_pred_equality(p, q);
}

// SHOULD FAIL: From p.entails(q), try to derive q.and(p) == p (wrong operand order matters for struct equality)
proof fn test_and_operand_order_matters(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures p == q.and(p),
{
    simplify_predicate(p, q);
}

}
