use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_filter's postcondition.
#[verifier::external_body]
proof fn vec_filter_spec(v: Seq<int>, f_spec: spec_fn(int) -> bool) -> (r: Seq<int>)
    ensures r.to_multiset() =~= v.to_multiset().filter(f_spec)
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Wrong result length — mutated to zero when matches exist.
// v = [1, 2, 3] with pred = x > 0 means all elements match.
// The result must have length 3, not 0.
// SHOULD FAIL
proof fn test_mutation_result_length_zero_when_all_match()
{
    let v: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    assert(r.len() == 0int);
}

// Test 2: Wrong result length — includes non-matching elements.
// v = [1, -1, 2], pred = x > 0. Only 1, 2 match → r.len() should be 2.
// Claiming r.len() == 3 (all elements kept) should be rejected.
// SHOULD FAIL
proof fn test_mutation_result_too_long()
{
    let v: Seq<int> = seq![1int, -1, 2];
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    assert(r.len() == 3int);
}

// Test 3: Wrong element multiplicity in result.
// v = [5, 5, 5], pred = x > 0. All match → r.to_multiset().count(5) == 3.
// Claiming count(5) == 2 (one element dropped) should be rejected.
// SHOULD FAIL
proof fn test_mutation_wrong_multiplicity()
{
    let v: Seq<int> = seq![5int, 5, 5];
    let pred = |x: int| x > 0;
    let r = vec_filter_spec(v, pred);
    assert(r.to_multiset().count(5int) == 2);
}

}
