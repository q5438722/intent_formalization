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

// ========== LOGICAL TESTS ==========

// Test 1: Cross-position equivalence — setting different positions with compatible
// values produces the same result. This is NOT guaranteed by the spec.
// set([1,3,5,7], 1, 4) -> [1,4,5,7]; set([1,3,5,7], 2, 4) -> [1,3,4,7].
// SHOULD FAIL
proof fn test_logical_cross_position_equivalence()
{
    let s: Seq<int> = seq![1int, 3, 5, 7];
    let r1 = set_spec(s, 1, 4); // [1, 4, 5, 7]
    let r2 = set_spec(s, 2, 4); // [1, 3, 4, 7]
    assert(r1 =~= r2); // different positions updated — not equivalent
}

// Test 2: Stronger inequality — assumes the result value is the midpoint of neighbors.
// set([1, 5, 9], 1, 3) -> [1, 3, 9]. The spec only requires s[0] < k < s[2],
// not that k == (s[0] + s[2]) / 2. Here 3 != (1+9)/2 = 5.
// SHOULD FAIL
proof fn test_logical_midpoint_assumption()
{
    let s: Seq<int> = seq![1int, 5, 9];
    let result = set_spec(s, 1, 3);
    assert(result[1] == (result[0] + result[2]) / 2); // 3 != 5
}

// Test 3: Determinism of choice — two different valid k values at the same index
// produce the same result. The spec allows any k satisfying the ordering constraints,
// so different k values must yield different results.
// set([1, 5, 9], 1, 3) -> [1, 3, 9]; set([1, 5, 9], 1, 7) -> [1, 7, 9].
// SHOULD FAIL
proof fn test_logical_different_k_same_result()
{
    let s: Seq<int> = seq![1int, 5, 9];
    let r1 = set_spec(s, 1, 3); // [1, 3, 9]
    let r2 = set_spec(s, 1, 7); // [1, 7, 9]
    assert(r1 =~= r2); // 3 ≠ 7 at index 1
}

}
