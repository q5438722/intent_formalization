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

// ===== BOUNDARY TESTS =====

// SHOULD FAIL: Missing backward entailment (q.entails(p))
// Only forward direction p.entails(q) is given, which is insufficient.
proof fn test_equality_missing_backward_entailment(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
    ensures
        p == q,
{
    // Cannot call temp_pred_equality — missing q.entails(p)
}

// SHOULD FAIL: Missing forward entailment (p.entails(q))
// Only backward direction q.entails(p) is given.
proof fn test_equality_missing_forward_entailment(p: TempPred<int>, q: TempPred<int>)
    requires
        q.entails(p),
    ensures
        p == q,
{
}

// SHOULD FAIL: implies_apply without p being satisfied
// Modus ponens requires both the implication AND p to hold.
proof fn test_implies_apply_missing_antecedent(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.implies(q).satisfied_by(ex),
        // missing: p.satisfied_by(ex)
    ensures
        q.satisfied_by(ex),
{
}

// SHOULD FAIL: implies_contraposition_apply without not(q) being satisfied
// Contraposition requires the implication AND the negation of the consequent.
proof fn test_contraposition_missing_negation(ex: Execution<int>, p: TempPred<int>, q: TempPred<int>)
    requires
        p.implies(q).satisfied_by(ex),
        // missing: not(q).satisfied_by(ex)
    ensures
        not(p).satisfied_by(ex),
{
}

// SHOULD FAIL: No preconditions at all — cannot derive equality
proof fn test_equality_no_preconditions(p: TempPred<int>, q: TempPred<int>)
    ensures
        p == q,
{
}

}
