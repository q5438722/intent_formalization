use vstd::prelude::*;

fn main() {}

verus! {

pub proof fn seq_equal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    ensures s1 == s2 <==> s1 + suffix == s2 + suffix
{
    assert_by(
        s1 == s2 ==> s1 + suffix == s2 + suffix,
        {
            if s1 == s2 {
                let len = s1.len();
                assert forall |i| 0<= i < (s1 + suffix).len() implies (#[trigger] (s1 + suffix)[i]) == (s2 + suffix)[i] by {
                    if i < len {
                        assert((s1 + suffix)[i] == s1[i]);
                        assert((s2 + suffix)[i] == s2[i]);
                    } else {
                        assert((s1 + suffix)[i] == suffix[i - len]);
                        assert((s2 + suffix)[i] == suffix[i - len]);
                    }
                }
            }
        }
    );
    assert_by(
        s1 + suffix == s2 + suffix ==> s1 == s2,
        {
            if s1 + suffix == s2 + suffix {
                assert((s1 + suffix).len() == (s2 + suffix).len());
                assert(s1.len() == s2.len());
                assert forall |i| 0<= i < s1.len() implies (#[trigger] s1[i]) == s2[i] by {
                    assert(s1[i] == (s1 + suffix)[i]);
                    assert(s2[i] == (s2 + suffix)[i]);
                }
                assert(s1 =~= s2);
            }
        }
    )
}

// ===== BOUNDARY TESTS =====
// These test edge cases and input validity at spec boundaries.

// SHOULD FAIL: Unequal sequences of same length cannot have equal concatenations
// Tests that the spec rejects: s1 != s2 ∧ s1 + suffix == s2 + suffix
proof fn boundary_test_1_unequal_seqs_equal_concat() {
    let s1: Seq<int> = seq![1];
    let s2: Seq<int> = seq![2];
    let suffix: Seq<int> = seq![3];
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix); // SHOULD FAIL
}

// SHOULD FAIL: Different-length sequences cannot be made equal by empty suffix
proof fn boundary_test_2_different_lengths_empty_suffix() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1];
    let suffix: Seq<int> = Seq::<int>::empty();
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 == s2); // SHOULD FAIL
}

// SHOULD FAIL: Spec should not allow concluding equality when suffixes differ
// Using same base seq but different suffixes should not yield equal concatenations
proof fn boundary_test_3_same_base_different_suffixes() {
    let s: Seq<int> = seq![1, 2];
    let suffix1: Seq<int> = seq![3];
    let suffix2: Seq<int> = seq![4];
    seq_equal_preserved_by_add(s, s, suffix1);
    assert(s + suffix1 == s + suffix2); // SHOULD FAIL
}

// SHOULD FAIL: Empty vs non-empty sequence should not be provable equal
proof fn boundary_test_4_empty_vs_nonempty() {
    let s1: Seq<int> = Seq::<int>::empty();
    let s2: Seq<int> = seq![1];
    let suffix: Seq<int> = seq![1];
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 == s2); // SHOULD FAIL
}

}
