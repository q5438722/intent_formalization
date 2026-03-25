use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::opaque]
pub open spec fn seq_is_unique<T>(s: Seq<T>) -> bool
{
    forall |i: int, j: int| #![trigger s[i], s[j]]
        0 <= i && i < s.len() && 0 <= j && j < s.len() && s[i] == s[j] ==> i == j
}

// Logical Test 1: seq_is_unique implies length <= 1 (stronger than spec guarantees).
// A unique seq can have length > 1, so this should fail.
// SHOULD FAIL
proof fn test_logical_unique_implies_at_most_one_element()
{
    reveal(seq_is_unique::<int>);
    let s: Seq<int> = seq![1, 2, 3];
    assert(seq_is_unique(s));
    assert(s.len() <= 1);
}

// Logical Test 2: Uniqueness preserved under concatenation.
// seq_is_unique(s1) && seq_is_unique(s2) does NOT imply seq_is_unique(s1 + s2).
// SHOULD FAIL
proof fn test_logical_concat_preserves_uniqueness()
{
    reveal(seq_is_unique::<int>);
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![2, 3];
    assert(seq_is_unique(s1));
    assert(seq_is_unique(s2));
    assert(seq_is_unique(s1 + s2));  // [1,2,2,3] has duplicate 2
}

// Logical Test 3: Pushing an element already present preserves uniqueness.
// This is NOT true: push(x) where x exists breaks uniqueness.
// SHOULD FAIL
proof fn test_logical_push_duplicate_preserves_uniqueness()
{
    reveal(seq_is_unique::<int>);
    let s: Seq<int> = seq![1, 2, 3];
    assert(seq_is_unique(s));
    let s2 = s.push(2);
    assert(seq_is_unique(s2));  // [1,2,3,2] has duplicate 2
}

}
