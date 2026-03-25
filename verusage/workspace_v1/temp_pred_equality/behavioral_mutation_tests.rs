use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from target =====

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

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
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
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_contraposition_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        not(q).satisfied_by(ex),
    ensures not(p).satisfied_by(ex),
{ unimplemented!() }

pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    assert forall |ex: Execution<T>| #[trigger] (p.pred)(ex) == (q.pred)(ex) by {
        if (p.pred)(ex) {
            implies_apply::<T>(ex, p, q);
        } else {
            implies_contraposition_apply::<T>(ex, q, p);
        }
    };
    assert(p.pred =~= q.pred);
}

// ===== BEHAVIORAL MUTATION TESTS =====

// SHOULD FAIL: Mutated conclusion — equality to negation instead of equality
// If p and q are mutually entailing, p == q holds, but p == not(q) should NOT.
proof fn test_equality_mutated_to_negation(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
    ensures
        p == not(q),
{
    temp_pred_equality(p, q);
}

// SHOULD FAIL: Mutated output of implies_apply — negated consequent
// Modus ponens yields q, not not(q). Asserting the negation is wrong.
proof fn test_implies_apply_negated_output(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures
        not(q).satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// SHOULD FAIL: Mutated output of contraposition — non-negated antecedent
// Contraposition yields not(p), not p. Asserting p is wrong.
proof fn test_contraposition_non_negated_output(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.implies(q).satisfied_by(ex),
        not(q).satisfied_by(ex),
    ensures
        p.satisfied_by(ex),
{
    implies_contraposition_apply(ex, p, q);
}

// SHOULD FAIL: Mutated entailment direction in equality — swapped arguments
// p.entails(q) and q.entails(p) should give p == q, but NOT not(p) == q
proof fn test_equality_mutated_to_negation_of_first(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
    ensures
        not(p) == q,
{
    temp_pred_equality(p, q);
}

// SHOULD FAIL: Mutated: implies_apply should not yield that p is unsatisfied
// Given p ==> q and p, we know q holds, but we should NOT conclude not(p).
proof fn test_implies_apply_mutated_to_negate_antecedent(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures
        not(p).satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

}
