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

// Test 1: Universal equality — any two executions are equal without precondition
// SHOULD FAIL: cannot prove equality without pointwise agreement
proof fn logical_test_universal_equality(ex1: Execution<int>, ex2: Execution<int>)
    ensures ex1 == ex2,
{
}

// Test 2: Self-equality implies constant function
// SHOULD FAIL: execution_equality(ex, ex) gives ex == ex (trivial), not that ex is constant
proof fn logical_test_self_equality_implies_constant(ex: Execution<int>)
    ensures
        forall |i: nat| #[trigger] (ex.nat_to_state)(i) == (ex.nat_to_state)(0),
{
    execution_equality(ex, ex);
}

// Test 3: Arbitrary execution must have a zero state
// SHOULD FAIL: no reason any execution must map some index to 0
proof fn logical_test_existence_of_zero(ex: Execution<int>)
    ensures
        exists |i: nat| #[trigger] (ex.nat_to_state)(i) == 0int,
{
}

}
