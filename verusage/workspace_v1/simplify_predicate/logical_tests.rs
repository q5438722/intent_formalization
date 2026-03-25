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

// ========== LOGICAL TESTS ==========
// These test properties NOT guaranteed by the spec and should FAIL verification.

// SHOULD FAIL: Entailment is NOT symmetric
proof fn test_entails_not_symmetric(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures q.entails(p),
{
}

// SHOULD FAIL: valid(p) cannot be assumed for arbitrary p
proof fn test_valid_arbitrary(p: TempPred<int>)
    ensures valid(p),
{
}

// SHOULD FAIL: Entailment is not provable for arbitrary predicates
proof fn test_entails_arbitrary(p: TempPred<int>, q: TempPred<int>)
    ensures p.entails(q),
{
}

// SHOULD FAIL: Transitivity cannot be used without the intermediate step
proof fn test_entails_skip_intermediate(p: TempPred<int>, q: TempPred<int>, r: TempPred<int>)
    requires p.entails(q),
    ensures p.entails(r),
{
}

// SHOULD FAIL: and-commutativity — p.and(q) == q.and(p) is not provable without extensionality proof
proof fn test_and_commutativity(p: TempPred<int>, q: TempPred<int>)
    ensures p.and(q) == q.and(p),
{
}

// SHOULD FAIL: Cross-function misuse — using simplify_predicate to derive that any entailed predicate
// is equal to the original (p == q from p.entails(q)), conflating p.and(q) with q
proof fn test_cross_function_misuse(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures p.and(q) == q,
{
    simplify_predicate(p, q);
    // simplify_predicate gives p == p.and(q), but not p.and(q) == q
}

}
