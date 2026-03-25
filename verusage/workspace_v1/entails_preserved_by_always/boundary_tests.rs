use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions (from target) ==========

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

// ========== Axioms (from target) ==========

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
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

// ========== Boundary Tests ==========

// SHOULD FAIL: Missing precondition — no p.entails(q) provided
// Tests whether the spec correctly requires the entailment precondition.
proof fn test_missing_precondition<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p).entails(always(q)),
{
    assert forall |ex| always(p).satisfied_by(ex) implies always(q).satisfied_by(ex) by {
        assert forall |i: nat| q.satisfied_by(#[trigger] ex.suffix(i)) by {
            always_unfold::<T>(ex, p);
            implies_apply::<T>(ex.suffix(i), p, q);
        };
    };
}

// SHOULD FAIL: Reversed precondition — q.entails(p) instead of p.entails(q)
// Tests whether swapping the entailment direction is rejected.
proof fn test_reversed_precondition<T>(p: TempPred<T>, q: TempPred<T>)
    requires q.entails(p),
    ensures always(p).entails(always(q)),
{
    assert forall |ex| always(p).satisfied_by(ex) implies always(q).satisfied_by(ex) by {
        assert forall |i: nat| q.satisfied_by(#[trigger] ex.suffix(i)) by {
            always_unfold::<T>(ex, p);
            implies_apply::<T>(ex.suffix(i), p, q);
        };
    };
}

// SHOULD FAIL: always_unfold without always(p) — only p.satisfied_by(ex)
// Tests whether a single satisfaction can substitute for always.
proof fn test_always_unfold_without_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// SHOULD FAIL: implies_apply without the antecedent p holding
// Tests whether modus ponens is enforced (need both implication and antecedent).
proof fn test_implies_apply_missing_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

}
