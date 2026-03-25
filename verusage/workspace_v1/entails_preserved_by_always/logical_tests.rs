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

// ========== Logical Tests ==========

// SHOULD FAIL: Entails is NOT symmetric
// p.entails(q) does not imply q.entails(p). Tests for unintended symmetry assumption.
proof fn test_entails_not_symmetric<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures q.entails(p),
{
}

// SHOULD FAIL: Self-entailment does NOT imply validity
// p.entails(p) is trivially true for any p, but valid(p) is NOT.
// If this passes, the spec confuses entailment with universal truth.
proof fn test_self_entailment_implies_valid<T>(p: TempPred<T>)
    requires p.entails(p),
    ensures valid(p),
{
}

// SHOULD FAIL: Local satisfaction does NOT imply global validity
// always(p) holding on ONE execution does not mean p holds on ALL executions.
proof fn test_local_always_implies_global_valid<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures valid(p),
{
}

// SHOULD FAIL: Conditional entailment does NOT collapse to unconditional validity
// always(p).entails(always(q)) only means: IF always(p) holds THEN always(q) holds.
// It does NOT mean valid(q).
proof fn test_entails_collapses_to_valid<T>(p: TempPred<T>, q: TempPred<T>)
    requires always(p).entails(always(q)),
    ensures valid(q),
{
}

}
