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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Wrong removed element — claims remove at index 1 yields element at index 0.
// s = [1, 3, 5], remove(1) should return 3 (s[1]), not 1 (s[0]).
// SHOULD FAIL
proof fn test_mutation_wrong_removed_element()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 1);
    assert(result.1 == 1int); // mutated: claims element is 1, actual is 3
}

// Test 2: Length unchanged after remove — claims result has same length as original.
// s = [1, 3, 5], remove(0) should yield length 2, not 3.
// SHOULD FAIL
proof fn test_mutation_length_unchanged()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 0);
    assert(result.0.len() == 3); // mutated: claims length 3, actual is 2
}

// Test 3: Removed element still in the result set.
// After removing s[1]=3 from [1,3,5], the result set should not contain 3.
// SHOULD FAIL
proof fn test_mutation_removed_element_still_in_set()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 1);
    assert(result.0.to_set().contains(3int)); // mutated: 3 was removed from the set
}

}
