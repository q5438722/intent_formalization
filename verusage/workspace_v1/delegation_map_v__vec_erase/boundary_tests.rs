use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_erase's postcondition.
// Body is opaque; only the ensures clause (the spec under test) is trusted.
#[verifier::external_body]
proof fn vec_erase_spec(v: Seq<int>, start: int, end: int) -> (r: Seq<int>)
    requires
        0 <= start <= end <= v.len(),
    ensures
        r == v.subrange(0, start) + v.subrange(end, v.len() as int),
{ unimplemented!() }

// ========== BOUNDARY TESTS ==========

// Test 1: start > end violates the precondition start <= end.
// The spec requires start <= end, so this call must be rejected.
// SHOULD FAIL
proof fn test_boundary_start_greater_than_end()
{
    let v: Seq<int> = seq![10int, 20, 30, 40, 50];
    let r = vec_erase_spec(v, 3, 1); // violates start <= end
}

// Test 2: end exceeds vector length, violating end <= v.len().
// v has 3 elements but we pass end = 5.
// SHOULD FAIL
proof fn test_boundary_end_exceeds_length()
{
    let v: Seq<int> = seq![10int, 20, 30];
    let r = vec_erase_spec(v, 1, 5); // violates end <= v.len()
}

// Test 3: Empty vector with nonzero erase range.
// v.len() == 0 but end == 1, violating end <= v.len().
// SHOULD FAIL
proof fn test_boundary_empty_vec_nonzero_range()
{
    let v: Seq<int> = Seq::empty();
    let r = vec_erase_spec(v, 0, 1); // violates end <= 0
}

}
