use vstd::prelude::*;

fn main() {}

verus! {

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    assert(ex1.nat_to_state =~= ex2.nat_to_state);
}

// Test 1: Partial equality — executions agree everywhere except at index 42
// SHOULD FAIL: precondition of execution_equality requires agreement on ALL indices
proof fn boundary_test_partial_equality(ex1: Execution<int>, ex2: Execution<int>)
    requires
        forall |i: nat| i != 42 ==> #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
        (ex1.nat_to_state)(42) != (ex2.nat_to_state)(42),
{
    execution_equality(ex1, ex2);
}

// Test 2: No relationship — arbitrary executions with no precondition
// SHOULD FAIL: precondition of execution_equality requires pointwise agreement
proof fn boundary_test_no_relation(ex1: Execution<int>, ex2: Execution<int>) {
    execution_equality(ex1, ex2);
}

// Test 3: Finite range agreement — only agree on first 100 indices
// SHOULD FAIL: precondition of execution_equality requires agreement on ALL indices
proof fn boundary_test_finite_agreement(ex1: Execution<int>, ex2: Execution<int>)
    requires
        forall |i: nat| i < 100 ==> #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
{
    execution_equality(ex1, ex2);
}

}
