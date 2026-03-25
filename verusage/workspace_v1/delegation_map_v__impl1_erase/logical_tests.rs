use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

spec fn sorted_int(s: Seq<int>) -> bool {
    forall |i: int, j: int| 0 <= i < j < s.len() ==> s[i] < s[j]
}

spec fn valid_int(s: Seq<int>) -> bool {
    sorted_int(s) && s.no_duplicates()
}

// Proof-level abstraction of remove's postconditions.
#[verifier::external_body]
proof fn remove_spec(s: Seq<int>, i: int) -> (result: (Seq<int>, int))
    requires
        valid_int(s),
        0 <= i < s.len(),
    ensures
        valid_int(result.0),
        result.1 == s[i],
        result.0 == s.remove(i),
        result.0.to_set() == s.to_set().remove(result.1),
{ unimplemented!() }

// Proof-level abstraction of erase's postconditions.
#[verifier::external_body]
proof fn erase_spec(s: Seq<int>, start: int, end: int) -> (result: Seq<int>)
    requires
        valid_int(s),
        0 <= start <= end <= s.len(),
    ensures
        valid_int(result),
        result == s.subrange(0, start) + s.subrange(end, s.len() as int),
        s.to_set() == result.to_set() + s.subrange(start, end).to_set(),
{ unimplemented!() }

// ========== LOGICAL TESTS ==========

// Test 1: Cross-function false equivalence.
// Claims that removing one element (remove at index 0) gives the same result
// as erasing two elements (erase(0, 2)). These are different operations:
// remove(0) on [1,3,5,7] -> [3,5,7]; erase(0,2) on [1,3,5,7] -> [5,7].
// SHOULD FAIL
proof fn test_logical_remove_neq_erase_two()
{
    let s: Seq<int> = seq![1int, 3, 5, 7];
    let r_remove = remove_spec(s, 0);
    let r_erase = erase_spec(s, 0, 2);
    assert(r_remove.0 =~= r_erase); // different number of elements removed
}

// Test 2: Structural assumption — contiguous integer range.
// After erasing middle elements, the result elements are not necessarily
// contiguous integers. erase([1,3,5,7], 1, 3) = [1,7], but 7 != 1+1.
// SHOULD FAIL
proof fn test_logical_contiguous_range_assumption()
{
    let s: Seq<int> = seq![1int, 3, 5, 7];
    let result = erase_spec(s, 1, 3); // erases [3, 5], result is [1, 7]
    assert(result[1] == result[0] + 1); // false: 7 != 2
}

// Test 3: Erasing all elements yields non-empty result.
// erase(s, 0, s.len()) removes everything. The result should be empty.
// Claiming it is non-empty tests an unwarranted global assumption.
// SHOULD FAIL
proof fn test_logical_erase_all_nonempty()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = erase_spec(s, 0, 3);
    assert(result.len() > 0); // false: result is Seq::empty()
}

}
