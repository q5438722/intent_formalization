use vstd::prelude::*;

fn main() {}

verus! {

// ============================================================
// Base definitions (from target file)
// ============================================================

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
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

pub proof fn entails_and_different_temp<T>(spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec1.entails(p),
        spec2.entails(q),
    ensures spec1.and(spec2).entails(p.and(q)),
{
    assert forall |ex| #[trigger] spec1.and(spec2).satisfied_by(ex) implies p.and(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec1, p);
        implies_apply::<T>(ex, spec2, q);
    };
}

// ============================================================
// Logical Tests: unintended reasoning / structural assumptions
// ============================================================

// Test 1: Entails is NOT symmetric - spec1.entails(p) does not imply p.entails(spec1)
// SHOULD FAIL
proof fn test_logical_entails_not_symmetric<T>(
    spec1: TempPred<T>, p: TempPred<T>
)
    requires
        spec1.entails(p),
    ensures p.entails(spec1),
{
}

// Test 2: Entailment does not imply universal validity of the consequent
// SHOULD FAIL
proof fn test_logical_valid_not_from_entails<T>(
    spec1: TempPred<T>, p: TempPred<T>
)
    requires
        spec1.entails(p),
    ensures valid(p),
{
}

// Test 3: Conjunction entailment does not extend to arbitrary predicates
// SHOULD FAIL
proof fn test_logical_entails_arbitrary_pred<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec1.entails(p),
        spec2.entails(q),
    ensures spec1.and(spec2).entails(r),
{
}

// Test 4: Cannot eliminate one conjunct from antecedent of entailment
// spec1.and(spec2).entails(p.and(q)) does NOT imply spec1.entails(p.and(q))
// SHOULD FAIL
proof fn test_logical_no_conjunct_elimination<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec1.and(spec2).entails(p.and(q)),
    ensures spec1.entails(p.and(q)),
{
}

// Test 5: No cross-transitivity between unrelated entailments
// spec1.entails(p) and spec2.entails(q) does NOT imply p.entails(q)
// SHOULD FAIL
proof fn test_logical_no_cross_transitivity<T>(
    spec1: TempPred<T>, spec2: TempPred<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        spec1.entails(p),
        spec2.entails(q),
    ensures p.entails(q),
{
}

}
