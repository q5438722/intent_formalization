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
// LOGICAL TESTS: Properties NOT explicitly guaranteed by spec
// Each test SHOULD FAIL verification.
// ============================================================

// Test 1: Assert result always equals low — not guaranteed when p(low) is false
// p(i) true for i >= 5, range [0,10). Smallest is 5, not 0.
// SHOULD FAIL
proof fn test_result_always_equals_low()
{
    let p = |i: int| -> bool { i >= 5 };
    assert(p(5));
    let res = choose_smallest(0, 10, p);
    assert(res == 0);
}

// Test 2: Assert result is strictly greater than low — not guaranteed when p(low) is true
// p(i) true for i >= 0, range [0,10). Smallest is 0 == low.
// SHOULD FAIL
proof fn test_result_strictly_greater_than_low()
{
    let p = |i: int| -> bool { i >= 0 };
    assert(p(0));
    let res = choose_smallest(0, 10, p);
    assert(res > 0);
}

// Test 3: Assert result is the ONLY satisfier — spec guarantees smallest, not uniqueness
// p(i) true for i >= 3, many satisfiers exist. Assert no other satisfier in range.
// SHOULD FAIL
proof fn test_result_is_unique_satisfier()
{
    let p = |i: int| -> bool { i >= 3 };
    assert(p(3));
    let res = choose_smallest(0, 10, p);
    assert(forall |i: int| #![trigger(p(i))] 0 <= i < 10 && i != res ==> !p(i));
}

// Test 4: Assert all values above result satisfy p — spec says nothing about values > res
// p is sparse: true only at 3 and 7. Values 4,5,6 don't satisfy p.
// SHOULD FAIL
proof fn test_all_above_result_satisfy_p()
{
    let p = |i: int| -> bool { i == 3 || i == 7 };
    assert(p(3));
    let res = choose_smallest(0, 10, p);
    assert(forall |i: int| #![trigger(p(i))] res < i < 10 ==> p(i));
}

} // verus!
