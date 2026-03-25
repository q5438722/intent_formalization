use vstd::prelude::*;

fn main() {}

verus!{

// ===== Base definitions from target file =====

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
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

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
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

// ===== LOGICAL TESTS =====
// These test properties NOT explicitly guaranteed by the spec.
// All should FAIL verification.

// SHOULD FAIL: always does NOT distribute over implies as an equality
// always(p => q) != always(p) => always(q) in general
// (the forward direction holds, but the reverse does not)
proof fn logical_test_1_always_distributes_over_implies<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.implies(q)) == always(p).implies(always(q)),
{
    temp_pred_equality::<T>(always(p.implies(q)), always(p).implies(always(q)));
}

// SHOULD FAIL: always(p) is NOT the same as p
// always wraps p with a universal quantifier over suffixes
proof fn logical_test_2_always_is_identity<T>(p: TempPred<T>)
    ensures always(p) == p,
{
    temp_pred_equality::<T>(always(p), p);
}

// SHOULD FAIL: arbitrary predicates under always are NOT equal
// This tests whether the axioms can be abused to equate unrelated predicates
proof fn logical_test_3_arbitrary_always_equality<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p) == always(q),
{
    temp_pred_equality::<T>(always(p), always(q));
}

}
