use vstd::prelude::*;

fn main() {}
verus! {

pub proof fn choose_smallest(low: int, high: int, p: spec_fn(int)->bool) -> (res:int)
  requires
    exists |i:int| #![trigger(p(i))] low <= i < high && p(i),
  ensures
    low <= res < high,
    p(res),
    forall |i:int| #![trigger(p(i))] low <= i < res ==> !p(i),
  decreases
    high - low,
{
  if p(low) {
    low
  } else {
    choose_smallest(low + 1, high, p)
  }
}

// ============================================================
// BOUNDARY TESTS: Violate preconditions with invalid inputs
// Each test SHOULD FAIL verification.
// ============================================================

// Test 1: Empty range (low == high) — no element can satisfy low <= i < high
// SHOULD FAIL
proof fn test_empty_range()
{
    let p = |i: int| -> bool { true };
    let res = choose_smallest(5, 5, p);
}

// Test 2: No satisfying element — p is always false in range
// SHOULD FAIL
proof fn test_no_satisfying_element()
{
    let p = |i: int| -> bool { false };
    let res = choose_smallest(0, 10, p);
}

// Test 3: Reversed range — low > high means empty range
// SHOULD FAIL
proof fn test_reversed_range()
{
    let p = |i: int| -> bool { true };
    let res = choose_smallest(10, 5, p);
}

// Test 4: Satisfying element exists but only outside the range
// SHOULD FAIL
proof fn test_satisfying_outside_range()
{
    let p = |i: int| -> bool { i == 20 };
    assert(p(20));  // witness exists globally but not in [0, 10)
    let res = choose_smallest(0, 10, p);
}

} // verus!
