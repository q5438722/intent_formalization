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

// ========== BOUNDARY TESTS ==========

// Test 1: Index == len (off-by-one, out of bounds).
// Precondition requires i < s.len(). Using i == s.len() violates this.
// SHOULD FAIL
proof fn test_boundary_index_eq_len()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 3, 4); // i == 3 == s.len(), violates i < 3
    assert(result.len() >= 0);
}

// Test 2: k equals predecessor (violates strict ordering with left neighbor).
// s = [1, 3, 5], set index 1 to 1. Requires s[0] < k, but 1 < 1 is false.
// SHOULD FAIL
proof fn test_boundary_k_equal_predecessor()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 1, 1); // s[0]=1, k=1, violates 1 < 1
    assert(result.len() >= 0);
}

// Test 3: k equals successor (violates strict ordering with right neighbor).
// s = [1, 3, 5], set index 1 to 5. Requires k < s[2], but 5 < 5 is false.
// SHOULD FAIL
proof fn test_boundary_k_equal_successor()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = set_spec(s, 1, 5); // k=5, s[2]=5, violates 5 < 5
    assert(result.len() >= 0);
}

}
