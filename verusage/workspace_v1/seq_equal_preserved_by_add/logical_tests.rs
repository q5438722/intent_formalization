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

// ===== LOGICAL TESTS =====
// Test properties NOT explicitly guaranteed by the spec.

// SHOULD FAIL: Prefix cancellation is NOT guaranteed by the suffix-cancellation spec.
// The spec only proves: s1 + suffix == s2 + suffix <==> s1 == s2
// It does NOT prove: prefix + s1 == prefix + s2 <==> s1 == s2
proof fn logical_test_1_prefix_cancellation_not_implied() {
    let s1: Seq<int> = seq![1];
    let s2: Seq<int> = seq![2];
    let prefix: Seq<int> = seq![0];
    // Only call the suffix spec — do NOT prove prefix cancellation independently
    seq_equal_preserved_by_add(s1, s2, Seq::<int>::empty());
    assert(prefix + s1 == prefix + s2); // SHOULD FAIL
}

// SHOULD FAIL: The spec does not imply suffix decomposition uniqueness.
// s1 + suffix1 == s2 + suffix2 does NOT imply suffix1 == suffix2
proof fn logical_test_2_suffix_decomposition_uniqueness() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1];
    let suffix1: Seq<int> = seq![3];
    let suffix2: Seq<int> = seq![2, 3];
    // s1 + suffix1 == [1,2,3] and s2 + suffix2 == [1,2,3] — equal!
    // But suffix1 != suffix2
    seq_equal_preserved_by_add(s1, s2, suffix1);
    assert(s1 + suffix1 == s2 + suffix2 ==> suffix1 == suffix2); // SHOULD FAIL
}

// SHOULD FAIL: Concatenation is NOT commutative — spec should not imply this
proof fn logical_test_3_commutativity_not_implied() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![3, 4];
    seq_equal_preserved_by_add(s1, s2, Seq::<int>::empty());
    assert(s1 + s2 == s2 + s1); // SHOULD FAIL
}

// SHOULD FAIL: The spec does not guarantee that appending preserves ordering/structure
// beyond equality. Try to derive a stronger property: element-wise relationship
proof fn logical_test_4_stronger_elementwise_claim() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![3, 4];
    let suffix: Seq<int> = seq![5];
    seq_equal_preserved_by_add(s1, s2, suffix);
    // Try to claim the first elements must be equal (they aren't)
    assert((s1 + suffix)[0] == (s2 + suffix)[0]); // SHOULD FAIL
}

}
