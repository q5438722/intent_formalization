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

// ==================== Behavioral Mutation Tests ====================

// Test 1: Reversed conclusion — try proving r.entails(p) instead of p.entails(r)
// SHOULD FAIL
proof fn mutation_reversed_conclusion<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures r.entails(p),
{
    entails_trans(p, q, r);
}

// Test 2: Partially reversed — try proving q.entails(p) from the same premises
// SHOULD FAIL
proof fn mutation_reverse_first_step<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures q.entails(p),
{
    entails_trans(p, q, r);
}

// Test 3: Strengthen to valid(q) from p.entails(q)
// SHOULD FAIL
proof fn mutation_strengthen_to_valid<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
    ensures valid(q),
{
}

// Test 4: Strengthen to valid(r) from entailment chain
// SHOULD FAIL
proof fn mutation_strengthen_chain_to_valid<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures valid(r),
{
    entails_trans(p, q, r);
}

// Test 5: Reverse last step — try r.entails(q) from the chain
// SHOULD FAIL
proof fn mutation_reverse_last_step<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures r.entails(q),
{
    entails_trans(p, q, r);
}

}
