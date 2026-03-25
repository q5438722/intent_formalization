use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_filter's postcondition.
// Body is opaque; only the ensures clause (the spec under test) is trusted.
#[verifier::external_body]
proof fn vec_filter_spec(v: Seq<int>, f_spec: spec_fn(int) -> bool) -> (r: Seq<int>)
    ensures r.to_multiset() =~= v.to_multiset().filter(f_spec)
{ unimplemented!() }

// ========== BOUNDARY TESTS ==========

// Test 1: Empty input must yield empty result.
// Filtering an empty multiset gives an empty multiset, so r must be empty.
// Claiming r is non-empty should be rejected.
// SHOULD FAIL
proof fn test_boundary_empty_input_nonempty_result()
{
    let v: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    assert(r.len() > 0);
}

// Test 2: Result cannot exceed input length.
// The filtered multiset has at most as many elements as the original.
// Claiming r.len() > v.len() should be rejected.
// SHOULD FAIL
proof fn test_boundary_result_longer_than_input()
{
    let v: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    assert(r.len() > v.len());
}

// Test 3: Predicate rejecting all elements must yield empty result.
// If no element satisfies f_spec, the filtered multiset is empty.
// Claiming r is non-empty should be rejected.
// SHOULD FAIL
proof fn test_boundary_all_rejected_nonempty_result()
{
    let v: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x < 0;
    let r = vec_filter_spec(v, pred);
    assert(r.len() > 0);
}

}
