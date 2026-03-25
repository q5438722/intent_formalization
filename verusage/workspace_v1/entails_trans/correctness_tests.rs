use vstd::prelude::*;

fn main() {}

verus! {

// ==================== Definitions (from target) ====================

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

pub proof fn entails_trans<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures p.entails(r),
{
    assert forall |ex| p.satisfied_by(ex) implies r.satisfied_by(ex) by {
        implies_apply::<T>(ex, p, q);
        implies_apply::<T>(ex, q, r);
    };
}

// =====================================================================
// BOUNDARY TESTS — Violate preconditions to check input rejection
// =====================================================================

// Test B1: entails_trans without p.entails(q)
// SHOULD FAIL
proof fn boundary_missing_first_precondition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires q.entails(r),
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test B2: entails_trans without q.entails(r)
// SHOULD FAIL
proof fn boundary_missing_second_precondition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires p.entails(q),
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test B3: entails_trans with no preconditions
// SHOULD FAIL
proof fn boundary_no_preconditions<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test B4: implies_apply without implication precondition
// SHOULD FAIL
proof fn boundary_implies_apply_missing_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// Test B5: implies_apply without satisfaction precondition
// SHOULD FAIL
proof fn boundary_implies_apply_missing_satisfaction<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// =====================================================================
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs/relations
// =====================================================================

// Test M1: Reversed conclusion — r.entails(p) instead of p.entails(r)
// SHOULD FAIL
proof fn mutation_reversed_conclusion<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures r.entails(p),
{
    entails_trans(p, q, r);
}

// Test M2: Partially reversed — q.entails(p) from chain
// SHOULD FAIL
proof fn mutation_reverse_first_step<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures q.entails(p),
{
    entails_trans(p, q, r);
}

// Test M3: Strengthen entailment to validity of consequent
// SHOULD FAIL
proof fn mutation_strengthen_to_valid<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures valid(q),
{
}

// Test M4: Strengthen chain to validity of final consequent
// SHOULD FAIL
proof fn mutation_strengthen_chain_to_valid<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures valid(r),
{
    entails_trans(p, q, r);
}

// Test M5: Reverse last step — r.entails(q) from chain
// SHOULD FAIL
proof fn mutation_reverse_last_step<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures r.entails(q),
{
    entails_trans(p, q, r);
}

// =====================================================================
// LOGICAL TESTS — Properties NOT explicitly guaranteed
// =====================================================================

// Test L1: Entailment is NOT symmetric
// SHOULD FAIL
proof fn logical_symmetry<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures q.entails(p),
{
}

// Test L2: Common cause does NOT distribute
// SHOULD FAIL
proof fn logical_common_cause<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        p.entails(r),
    ensures q.entails(r),
{
}

// Test L3: Entailment does NOT decompose through arbitrary intermediates
// SHOULD FAIL
proof fn logical_decomposition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires p.entails(r),
    ensures p.entails(q),
{
}

// Test L4: Entailment does NOT imply validity of antecedent
// SHOULD FAIL
proof fn logical_validity_backward<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures valid(p),
{
}

// Test L5: Common consequent does NOT link sources
// SHOULD FAIL
proof fn logical_wrong_chaining<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        r.entails(q),
    ensures p.entails(r),
{
}

}
