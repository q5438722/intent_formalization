use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::opaque]
pub open spec fn seq_is_unique<T>(s: Seq<T>) -> bool
{
    forall |i: int, j: int| #![trigger s[i], s[j]]
        0 <= i && i < s.len() && 0 <= j && j < s.len() && s[i] == s[j] ==> i == j
}

// Boundary Test 1: Empty sequence should be unique (vacuously true).
// Asserting it is NOT unique should fail.
// SHOULD FAIL
proof fn test_boundary_empty_seq_not_unique()
{
    let s: Seq<int> = seq![];
    reveal(seq_is_unique::<int>);
    assert(!seq_is_unique(s));
}

// Boundary Test 2: Single-element sequence should be unique.
// Asserting it is NOT unique should fail.
// SHOULD FAIL
proof fn test_boundary_single_element_not_unique()
{
    let s: Seq<int> = seq![42];
    reveal(seq_is_unique::<int>);
    assert(!seq_is_unique(s));
}

// Boundary Test 3: Minimum-size non-unique sequence [0, 0].
// Asserting it IS unique should fail (boundary between unique and non-unique).
// SHOULD FAIL
proof fn test_boundary_min_non_unique_claimed_unique()
{
    let s: Seq<int> = seq![0, 0];
    reveal(seq_is_unique::<int>);
    assert(seq_is_unique(s));
}

}
