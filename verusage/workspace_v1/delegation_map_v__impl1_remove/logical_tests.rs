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

// ========== LOGICAL TESTS ==========

// Test 1: Cross-index equivalence — claims removing at different indices
// yields the same result. remove(s, 0) on [1,3,5] = [3,5], but
// remove(s, 2) on [1,3,5] = [1,3]. These are different.
// SHOULD FAIL
proof fn test_logical_different_indices_same_result()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let r0 = remove_spec(s, 0);
    let r2 = remove_spec(s, 2);
    assert(r0.0 =~= r2.0); // false: [3,5] != [1,3]
}

// Test 2: Structural assumption — claims removing the first element
// preserves the first element's value. remove(s, 0) on [1,3,5] = [3,5],
// so result[0] = 3, not 1.
// SHOULD FAIL
proof fn test_logical_remove_first_preserves_head()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 0);
    assert(result.0[0] == s[0]); // false: result[0] = 3, s[0] = 1
}

// Test 3: Global assumption — removing from a singleton gives non-empty result.
// s = [42], remove(0) = [], so result length is 0.
// SHOULD FAIL
proof fn test_logical_singleton_remove_nonempty()
{
    let s: Seq<int> = seq![42int];
    let result = remove_spec(s, 0);
    assert(result.0.len() > 0); // false: result is empty
}

}
