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

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
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

// ========== Boundary Tests ==========

// SHOULD FAIL
// Test 1: Call execution_equality without satisfying its precondition.
// Two arbitrary executions should NOT be proven equal without pointwise equality.
proof fn test_execution_equality_no_precondition<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
}

// SHOULD FAIL
// Test 2: Call temp_pred_equality with only ONE direction of entailment.
// Mutual entailment is required; one direction alone should be insufficient.
proof fn test_temp_pred_equality_one_direction<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures p == q,
{
    temp_pred_equality(p, q);
}

// SHOULD FAIL
// Test 3: Suffix composition with multiplication instead of addition.
// ex.suffix(i).suffix(j) == ex.suffix(i + j), NOT ex.suffix(i * j).
proof fn test_suffix_composition_multiply<T>(ex: Execution<T>, i: nat, j: nat)
    ensures ex.suffix(i).suffix(j) == ex.suffix(i * j),
{
    execution_equality(ex.suffix(i).suffix(j), ex.suffix(i * j));
}

}
