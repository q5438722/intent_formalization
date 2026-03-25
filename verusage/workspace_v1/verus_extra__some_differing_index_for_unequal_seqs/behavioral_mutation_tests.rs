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

// === BEHAVIORAL MUTATION TESTS ===

// Mutation Test 1: Assert elements are EQUAL at the returned index (negates s1[i] != s2[i])
// SHOULD FAIL
proof fn test_mutation_equal_at_index() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 5, 3];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    assert(s1[i] == s2[i]);
}

// Mutation Test 2: Assert returned index is out of bounds (negates i < s1.len())
// SHOULD FAIL
proof fn test_mutation_index_out_of_bounds() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 5, 3];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    assert(i >= s1.len());
}

// Mutation Test 3: Assert returned index is negative (negates 0 <= i)
// SHOULD FAIL
proof fn test_mutation_negative_index() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 5, 3];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    assert(i < 0);
}

// Mutation Test 4: Assert a wrong specific index when only one position differs
// (s1=[1,2,3], s2=[1,2,4] differ only at index 2; asserting i==0 is wrong)
// SHOULD FAIL
proof fn test_mutation_wrong_specific_index() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 4];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    assert(i == 0);
}

}
