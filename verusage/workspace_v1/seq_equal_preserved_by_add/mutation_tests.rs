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

// ===== BEHAVIORAL MUTATION TESTS =====
// Start from valid inputs, mutate expected outputs or relations.

// SHOULD FAIL: Mutate the forward direction — equal seqs should yield equal concats,
// but we assert the opposite (not equal)
proof fn mutation_test_1_equal_seqs_unequal_concat() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 2];
    let suffix: Seq<int> = seq![3];
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix != s2 + suffix); // SHOULD FAIL
}

// SHOULD FAIL: Mutate the backward direction — equal concats should imply equal seqs,
// but we assert the seqs are not equal despite equal concatenations
proof fn mutation_test_2_equal_concat_unequal_seqs() {
    let s1: Seq<int> = seq![5, 6];
    let s2: Seq<int> = seq![5, 6];
    let suffix: Seq<int> = seq![7, 8];
    seq_equal_preserved_by_add(s1, s2, suffix);
    // Concat is equal (since s1 == s2), but claim seqs are different
    assert(s1 != s2); // SHOULD FAIL
}

// SHOULD FAIL: Mutate output — claim that unequal seqs produce equal concatenations
proof fn mutation_test_3_unequal_seqs_forced_equal_concat() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 4];
    let suffix: Seq<int> = seq![5];
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix); // SHOULD FAIL
}

// SHOULD FAIL: Mutate relation — swap the role of suffix and base
// Claim s1 + suffix == suffix + s2 (wrong operation order)
proof fn mutation_test_4_swapped_operand_order() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 2];
    let suffix: Seq<int> = seq![3, 4];
    seq_equal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == suffix + s2); // SHOULD FAIL
}

}
