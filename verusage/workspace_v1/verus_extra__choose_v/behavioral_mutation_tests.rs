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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// Each test SHOULD FAIL verification.
// ============================================================

// Test 1: Assert result is NOT the correct smallest — mutate expected value
// p(i) true for i >= 3, so smallest in [0,10) is 3. Assert res != 3.
// SHOULD FAIL
proof fn test_result_is_not_smallest()
{
    let p = |i: int| -> bool { i >= 3 };
    assert(p(3));
    let res = choose_smallest(0, 10, p);
    assert(res != 3);
}

// Test 2: Assert result does NOT satisfy the predicate — negate p(res)
// SHOULD FAIL
proof fn test_result_does_not_satisfy_predicate()
{
    let p = |i: int| -> bool { i == 5 };
    assert(p(5));
    let res = choose_smallest(0, 10, p);
    assert(!p(res));
}

// Test 3: Assert result is the LARGEST satisfier instead of smallest
// p(i) true for 3 <= i < 8, so largest is 7, smallest is 3. Assert res == 7.
// SHOULD FAIL
proof fn test_result_is_largest_not_smallest()
{
    let p = |i: int| -> bool { 3 <= i && i < 8 };
    assert(p(3));
    let res = choose_smallest(0, 10, p);
    assert(res == 7);
}

// Test 4: Assert result is out of range (>= high)
// SHOULD FAIL
proof fn test_result_out_of_range()
{
    let p = |i: int| -> bool { i >= 0 };
    assert(p(0));
    let res = choose_smallest(0, 10, p);
    assert(res >= 10);
}

} // verus!
