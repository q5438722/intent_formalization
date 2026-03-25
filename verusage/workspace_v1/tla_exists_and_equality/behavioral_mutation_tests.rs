use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions from target file ===

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

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn tla_exists_proved_by_witness<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>, witness_a: A)
    requires a_to_p(witness_a).satisfied_by(ex),
    ensures tla_exists(a_to_p).satisfied_by(ex),
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

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: Mutated equality — drops .and(q) from the RHS.
// The correct identity is: tla_exists(|a| P(a) ∧ Q) == tla_exists(P) ∧ Q
// Here we assert the WRONG version: tla_exists(|a| P(a) ∧ Q) == tla_exists(P)
// SHOULD FAIL
proof fn test_mutation_drop_and_q(a_to_p: spec_fn(int) -> TempPred<int>, q: TempPred<int>)
{
    assert(tla_exists(|a: int| a_to_p(a).and(q)) == tla_exists(a_to_p));
}

// Test 2: Existential satisfied does NOT mean a specific witness works.
// tla_exists(a_to_p).satisfied_by(ex) only guarantees SOME a works, not a=0.
// SHOULD FAIL
proof fn test_mutation_reverse_witness(ex: Execution<int>, a_to_p: spec_fn(int) -> TempPred<int>)
    requires tla_exists(a_to_p).satisfied_by(ex),
{
    assert(a_to_p(0int).satisfied_by(ex));
}

// Test 3: The witness lemma proves existential for ONE execution.
// Asserting validity (for ALL executions) is an invalid strengthening.
// SHOULD FAIL
proof fn test_mutation_strengthen_to_valid(ex: Execution<int>, a_to_p: spec_fn(int) -> TempPred<int>, witness_a: int)
    requires a_to_p(witness_a).satisfied_by(ex),
{
    tla_exists_proved_by_witness::<int, int>(ex, a_to_p, witness_a);
    // postcondition only gives: tla_exists(a_to_p).satisfied_by(ex) for THIS ex
    assert(valid(tla_exists(a_to_p)));
}

}
