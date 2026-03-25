use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_filter's postcondition.
#[verifier::external_body]
proof fn vec_filter_spec(v: Seq<int>, f_spec: spec_fn(int) -> bool) -> (r: Seq<int>)
    ensures r.to_multiset() =~= v.to_multiset().filter(f_spec)
{ unimplemented!() }

// ========== LOGICAL TESTS ==========

// Test 1: Order preservation — NOT entailed by multiset equality.
// The spec guarantees multiset equality, not sequence equality.
// Claiming r equals the sequence-level filter (which preserves order) should fail
// because multiset equality allows arbitrary permutations of matching elements.
// SHOULD FAIL
proof fn test_logical_order_preservation()
{
    let v: Seq<int> = seq![3int, 1, 2];
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    // v.filter(pred) preserves order: [3, 1, 2]
    // But r could be any permutation: [1, 2, 3], [2, 3, 1], etc.
    assert(r =~= v.filter(pred));
}

// Test 2: Determinism — two calls with same arguments must produce same result.
// The spec allows any sequence whose multiset matches, so different calls
// could return different permutations. Asserting equality should fail.
// SHOULD FAIL
proof fn test_logical_determinism()
{
    let v: Seq<int> = seq![2int, 1, 3];
    let pred = |x: int| x > 0;
    let r1 = vec_filter_spec(v, pred);
    let r2 = vec_filter_spec(v, pred);
    assert(r1 =~= r2);
}

// Test 3: Specific element at specific index — NOT entailed by multiset equality.
// v = [3, 1, 2], pred = x > 1 → matching elements are {3, 2}.
// The spec doesn't fix the order, so we can't determine r[0].
// SHOULD FAIL
proof fn test_logical_first_element_determined()
{
    let v: Seq<int> = seq![3int, 1, 2];
    let pred = |x: int| x > 1;
    let r = vec_filter_spec(v, pred);
    // r has 2 elements (3 and 2), but order is unspecified by multiset equality
    assert(r[0] == 3int);
}

}
