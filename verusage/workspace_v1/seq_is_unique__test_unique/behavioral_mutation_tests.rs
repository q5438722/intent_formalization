use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::opaque]
pub open spec fn seq_is_unique<T>(s: Seq<T>) -> bool
{
    forall |i: int, j: int| #![trigger s[i], s[j]]
        0 <= i && i < s.len() && 0 <= j && j < s.len() && s[i] == s[j] ==> i == j
}

// Behavioral Mutation 1: Sequence with adjacent duplicates [1, 1].
// Mutate expected output: claim it IS unique.
// SHOULD FAIL
proof fn test_mutation_adjacent_duplicates_unique()
{
    let s: Seq<int> = seq![1, 1];
    reveal(seq_is_unique::<int>);
    assert(seq_is_unique(s));
}

// Behavioral Mutation 2: Sequence with distinct elements [1, 2].
// Mutate expected output: claim it is NOT unique.
// SHOULD FAIL
proof fn test_mutation_distinct_pair_not_unique()
{
    let s: Seq<int> = seq![1, 2];
    reveal(seq_is_unique::<int>);
    assert(!seq_is_unique(s));
}

// Behavioral Mutation 3: Sequence with non-adjacent duplicates [1, 2, 1].
// Mutate expected output: claim it IS unique.
// SHOULD FAIL
proof fn test_mutation_non_adjacent_duplicates_unique()
{
    let s: Seq<int> = seq![1, 2, 1];
    reveal(seq_is_unique::<int>);
    assert(seq_is_unique(s));
}

}
