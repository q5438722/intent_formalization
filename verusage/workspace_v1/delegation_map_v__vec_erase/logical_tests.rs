use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_erase's postcondition.
#[verifier::external_body]
proof fn vec_erase_spec(v: Seq<int>, start: int, end: int) -> (r: Seq<int>)
    requires
        0 <= start <= end <= v.len(),
    ensures
        r == v.subrange(0, start) + v.subrange(end, v.len() as int),
{ unimplemented!() }

// ========== LOGICAL TESTS ==========

// Test 1: Stronger inequality — no-op erase (start == end) should NOT shorten the vector.
// When start == end, nothing is erased, so r.len() == v.len().
// Asserting r.len() < v.len() is a false stronger claim.
// SHOULD FAIL
proof fn test_logical_noop_erase_shortens()
{
    let v: Seq<int> = seq![10int, 20, 30];
    let r = vec_erase_spec(v, 1, 1);
    // r == v.subrange(0,1) + v.subrange(1,3) == [10] + [20,30] == [10,20,30]
    assert(r.len() < v.len());
}

// Test 2: Cross-input reasoning — different vectors with same erase range
// must NOT produce the same result if they differ outside the erased region.
// v = [10, 20, 30], w = [99, 20, 30], erase [1, 2).
// r_v = [10, 30], r_w = [99, 30]. These are different.
// SHOULD FAIL
proof fn test_logical_different_inputs_same_output()
{
    let v: Seq<int> = seq![10int, 20, 30];
    let w: Seq<int> = seq![99int, 20, 30];
    let r_v = vec_erase_spec(v, 1, 2);
    let r_w = vec_erase_spec(w, 1, 2);
    assert(r_v =~= r_w);
}

// Test 3: Idempotency — erasing the same range twice does NOT yield the same result
// as erasing it once, because the second erase operates on a shorter vector.
// v = [10, 20, 30, 40, 50], erase [1, 3) → r1 = [10, 40, 50] (len 3).
// erase [1, 3) on r1 → r2 = [10] (len 1). r1 ≠ r2.
// SHOULD FAIL
proof fn test_logical_idempotency()
{
    let v: Seq<int> = seq![10int, 20, 30, 40, 50];
    let r1 = vec_erase_spec(v, 1, 3);
    let r2 = vec_erase_spec(r1, 1, 3);
    assert(r1 =~= r2);
}

}
