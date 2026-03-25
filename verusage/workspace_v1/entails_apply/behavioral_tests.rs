use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from target file =====

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

proof fn entails_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// ===== Behavioral Mutation Tests =====

// SHOULD FAIL: Mutate postcondition to claim q is universally valid (too strong)
proof fn mutation_test_entails_implies_valid<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures valid(q), // mutated: q.satisfied_by(ex) -> valid(q)
{
    entails_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Negate the conclusion - claim q is NOT satisfied
proof fn mutation_test_negated_conclusion<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures !q.satisfied_by(ex), // mutated: negated
{
    entails_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Reverse the entailment direction in conclusion
proof fn mutation_test_reversed_entailment<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.entails(p), // mutated: reversed direction
{
    entails_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Strengthen postcondition of implies_apply to claim universal validity
proof fn mutation_test_implies_to_valid<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures valid(q), // mutated: too strong
{
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Claim that p.satisfied_by(ex) also holds for an arbitrary different execution
proof fn mutation_test_satisfaction_transfers<T>(ex: Execution<T>, ex2: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex2), // mutated: different execution
{
    entails_apply::<T>(ex, p, q);
}

}
