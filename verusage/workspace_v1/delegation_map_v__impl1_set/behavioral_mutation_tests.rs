use vstd::prelude::*;

fn main() {}

verus! {

spec fn sorted_int(s: Seq<int>) -> bool {
    forall |i: int, j: int| 0 <= i < j < s.len() ==> s[i] < s[j]
}

spec fn valid_int(s: Seq<int>) -> bool {
    sorted_int(s) && s.no_duplicates()
}

// Proof-level abstraction of set's postconditions.
#[verifier::external_body]
proof fn set_spec(s: Seq<int>, i: int, k: int) -> (result: Seq<int>)
    requires
        valid_int(s),
        0 <= i < s.len(),
        i > 0 ==> s[i - 1] < k,
        i < s.len() - 1 ==> k < s[i + 1],
    ensures
        valid_int(result),
        result == s.update(i, k),
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Wrong update position — claims k appears at index 0 instead of index 1.
// s = [1, 3, 5], set(s, 1, 4) should produce [1, 4, 5]. result[0] should be 1, not 4.
// SHOULD FAIL
proof fn test_mutation_wrong_update_position()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 1, 4);
    assert(result[0] == 4); // mutated: k should be at index 1, not 0
}

// Test 2: Length changed — claims result has different length after set.
// set replaces an element in place, so length should be unchanged (3, not 2).
// SHOULD FAIL
proof fn test_mutation_length_changed()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 1, 4);
    assert(result.len() == 2); // mutated: claims length 2, actual is 3
}

// Test 3: Result unchanged — claims result equals original after updating with a different value.
// s = [1, 3, 5], set(s, 1, 4) produces [1, 4, 5] ≠ [1, 3, 5].
// SHOULD FAIL
proof fn test_mutation_result_unchanged()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 1, 4);
    assert(result =~= s); // mutated: claims result equals original
}

}
