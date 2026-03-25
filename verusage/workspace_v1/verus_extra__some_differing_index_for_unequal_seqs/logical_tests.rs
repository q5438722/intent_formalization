use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// --- Target function under test ---
pub proof fn some_differing_index_for_unequal_seqs<A>(s1: Seq<A>, s2: Seq<A>) -> (i: int)
  requires
    s1 != s2,
    s1.len() == s2.len(),
  ensures
    0 <= i < s1.len(),
    s1[i] != s2[i],
{
  if forall |i| 0 <= i < s1.len() ==> s1[i] == s2[i] {
    assert(s1 =~= s2);
  }
  choose |i:int| 0 <= i < s1.len() && s1[i] != s2[i]
}

// === LOGICAL TESTS ===

// Logical Test 1: Assert the returned index is always the minimum differing index
// The spec does NOT guarantee minimality — only that SOME differing index is returned
// SHOULD FAIL
proof fn test_logical_minimum_index() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5, 6];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    // Assert no earlier index differs (i.e., i is the first differing position)
    assert(forall |j: int| 0 <= j < i ==> s1[j] == s2[j]);
}

// Logical Test 2: Assert that only ONE position differs (uniqueness of difference)
// The spec guarantees at least one differing position, not that it is the only one
// SHOULD FAIL
proof fn test_logical_unique_difference() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5, 6];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    // Assert all OTHER positions are equal (false: all 3 positions differ)
    assert(forall |j: int| 0 <= j < s1.len() && j != i ==> s1[j] == s2[j]);
}

// Logical Test 3: Assert a stronger ordering relation (s1[i] > s2[i])
// The spec only guarantees s1[i] != s2[i], not any particular ordering
// SHOULD FAIL
proof fn test_logical_stronger_inequality() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5, 6];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    // Assert s1[i] > s2[i] — this is false here since s1 elements < s2 elements
    assert(s1[i] > s2[i]);
}

// Logical Test 4: Assert commutativity — swapped arguments yield the same index
// The spec does NOT guarantee this
// SHOULD FAIL
proof fn test_logical_commutativity() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5, 6];
    let i1 = some_differing_index_for_unequal_seqs(s1, s2);
    let i2 = some_differing_index_for_unequal_seqs(s2, s1);
    assert(i1 == i2);
}

}
