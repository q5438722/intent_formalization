use vstd::prelude::*;

fn main() {}

verus!{

// Original lemma under test
pub proof fn seq_equal_preserved_by_add_prefix<A>(prefix: Seq<A>, s1: Seq<A>, s2: Seq<A>)
    ensures s1 == s2 <==> prefix + s1 == prefix + s2
{
    assert_by(
        s1 == s2 ==> prefix + s1 == prefix + s2,
        {
            if s1 == s2 {
                let len = prefix.len();
                assert forall |i| 0<= i < (prefix + s1).len() implies (#[trigger] (prefix + s1)[i]) == (prefix + s2)[i] by {
                    if i < len {
                        assert((prefix + s1)[i] == prefix[i]);
                        assert((prefix + s2)[i] == prefix[i]);
                    } else {
                        assert((prefix + s1)[i] == s1[i - len]);
                        assert((prefix + s2)[i] == s2[i - len]);
                    }
                }
            }
        }
    );
    assert_by(
        prefix + s1 == prefix + s2 ==> s1 == s2,
        {
            if prefix + s1 == prefix + s2 {
                assert((prefix + s1).len() == (prefix + s2).len());
                assert(s1.len() == s2.len());
                let len = prefix.len();
                assert forall |i| 0<= i < s1.len() implies (#[trigger] s1[i]) == s2[i] by {
                    assert(s1[i] == (prefix + s1)[i + len]);
                    assert(s2[i] == (prefix + s2)[i + len]);
                }
                assert(s1 =~= s2);
            }
        }
    )
}

// === BOUNDARY TESTS ===
// These tests violate or stress the semantic boundaries with edge-case inputs.

// SHOULD FAIL
// Boundary Test 1: Unequal sequences with empty prefix — assert concatenations are equal.
// With empty prefix, prefix + s1 == s1 and prefix + s2 == s2, so this should fail since s1 != s2.
proof fn test_boundary_empty_prefix_unequal_seqs()
{
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![4, 5, 6];
    let prefix: Seq<int> = Seq::empty();
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert(prefix + s1 == prefix + s2); // SHOULD FAIL
}

// SHOULD FAIL
// Boundary Test 2: Sequences of different lengths — assert equality after prefix.
// s1 has length 2, s2 has length 3, so s1 != s2 and prefix + s1 != prefix + s2.
proof fn test_boundary_different_lengths()
{
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 2, 3];
    let prefix: Seq<int> = seq![0];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert(s1 == s2); // SHOULD FAIL
}

// SHOULD FAIL
// Boundary Test 3: Nonempty vs empty sequence — assert concatenations are equal.
// s1 = [1], s2 = [], they have different lengths so cannot be equal.
proof fn test_boundary_nonempty_vs_empty()
{
    let s1: Seq<int> = seq![1];
    let s2: Seq<int> = Seq::empty();
    let prefix: Seq<int> = seq![10, 20];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert(prefix + s1 == prefix + s2); // SHOULD FAIL
}

}
