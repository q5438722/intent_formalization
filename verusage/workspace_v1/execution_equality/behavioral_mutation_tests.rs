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

// Test 1: Assert inequality after valid equality proof
// SHOULD FAIL: ex1 == ex2 is proven, so ex1 != ex2 is false
proof fn mutation_test_assert_inequality(ex1: Execution<int>, ex2: Execution<int>)
    requires
        forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
{
    execution_equality(ex1, ex2);
    assert(ex1 != ex2);
}

// Test 2: Assert wrong state value after proving equality
// SHOULD FAIL: ex1(0) == 0 and ex1(0) == ex2(0) imply ex2(0) == 0, not 1
proof fn mutation_test_wrong_state_value(ex1: Execution<int>, ex2: Execution<int>)
    requires
        forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
        (ex1.nat_to_state)(0) == 0int,
{
    execution_equality(ex1, ex2);
    assert((ex2.nat_to_state)(0) == 1int);
}

// Test 3: Assert equality with unrelated third execution
// SHOULD FAIL: ex3 has no established relationship with ex1 or ex2
proof fn mutation_test_unrelated_third(ex1: Execution<int>, ex2: Execution<int>, ex3: Execution<int>)
    requires
        forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
{
    execution_equality(ex1, ex2);
    assert(ex1 == ex3);
}

}
