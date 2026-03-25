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

// The original lemma under test
pub proof fn entails_preserved_by_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures always(p).entails(always(q)),
{
    assert forall |ex| always(p).satisfied_by(ex) implies always(q).satisfied_by(ex) by {
        assert forall |i:nat| q.satisfied_by(#[trigger] ex.suffix(i)) by {
            always_unfold::<T>(ex, p);
            implies_apply::<T>(ex.suffix(i), p, q);
        };
    };
}

// ========== Boundary Tests ==========

// SHOULD FAIL: Missing precondition — no p.entails(q) provided
proof fn test_boundary_missing_precondition<T>(p: TempPred<T>, q: TempPred<T>)
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
proof fn test_boundary_reversed_precondition<T>(p: TempPred<T>, q: TempPred<T>)
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
proof fn test_boundary_unfold_without_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// SHOULD FAIL: implies_apply without the antecedent p holding
proof fn test_boundary_modus_ponens_no_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// ========== Behavioral Mutation Tests ==========

// SHOULD FAIL: Reversed conclusion direction
proof fn test_mutation_reversed_conclusion<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures always(q).entails(always(p)),
{
    assert forall |ex| always(q).satisfied_by(ex) implies always(p).satisfied_by(ex) by {
        assert forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)) by {
            always_unfold::<T>(ex, q);
            implies_apply::<T>(ex.suffix(i), q, p);
        };
    };
}

// SHOULD FAIL: Strengthened conclusion — p entails always(q)
proof fn test_mutation_strengthen_to_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures p.entails(always(q)),
{
}

// SHOULD FAIL: Mutated to unconditional validity
proof fn test_mutation_to_valid<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures valid(always(q)),
{
}

// ========== Logical Tests ==========

// SHOULD FAIL: Entails is NOT symmetric
proof fn test_logical_entails_not_symmetric<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures q.entails(p),
{
}

// SHOULD FAIL: Self-entailment does NOT imply validity
proof fn test_logical_self_entails_valid<T>(p: TempPred<T>)
    requires p.entails(p),
    ensures valid(p),
{
}

// SHOULD FAIL: Local satisfaction does NOT imply global validity
proof fn test_logical_local_to_global<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures valid(p),
{
}

// SHOULD FAIL: Conditional entailment does NOT collapse to unconditional validity
proof fn test_logical_entails_not_valid<T>(p: TempPred<T>, q: TempPred<T>)
    requires always(p).entails(always(q)),
    ensures valid(q),
{
}

}
