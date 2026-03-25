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

// === BOUNDARY TESTS ===

// Boundary Test 1: Equal sequences — violates requires s1 != s2
// SHOULD FAIL
proof fn test_boundary_equal_sequences() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 3];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
}

// Boundary Test 2: Different-length sequences — violates requires s1.len() == s2.len()
// SHOULD FAIL
proof fn test_boundary_different_lengths() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
}

// Boundary Test 3: Empty equal sequences — violates requires s1 != s2
// SHOULD FAIL
proof fn test_boundary_empty_equal() {
    let s1: Seq<int> = Seq::empty();
    let s2: Seq<int> = Seq::empty();
    let i = some_differing_index_for_unequal_seqs(s1, s2);
}

// Boundary Test 4: One empty, one non-empty — violates requires s1.len() == s2.len()
// SHOULD FAIL
proof fn test_boundary_length_mismatch_empty_nonempty() {
    let s1: Seq<int> = seq![1];
    let s2: Seq<int> = Seq::empty();
    let i = some_differing_index_for_unequal_seqs(s1, s2);
}

}
