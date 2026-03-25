use vstd::prelude::*;

fn main() {}

verus! {

pub proof fn seq_unequal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    requires s1 != s2
    ensures s1 + suffix != s2 + suffix
{
    assert(!(s1 =~= s2));
    if s1.len() == s2.len() {
        let witness_idx = choose |i: int| 0 <= i < s1.len() && s1[i] != s2[i];
        assert((s1 + suffix)[witness_idx] != (s2 + suffix)[witness_idx]);
    } else {
        assert((s1 + suffix).len() != (s2 + suffix).len());
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====
// Start from valid inputs (s1 != s2), mutate the expected output relation.
// All should FAIL verification because the asserted property contradicts the postcondition.

// SHOULD FAIL: Negate postcondition — assert concatenations ARE equal despite s1 != s2
proof fn mutation_test_1_assert_concat_equal() {
    let s1: Seq<int> = seq![1, 0];
    let s2: Seq<int> = seq![2, 0];
    let suffix: Seq<int> = seq![3];
    assert(s1[0] != s2[0]); // hint: prove s1 != s2
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix); // SHOULD FAIL: contradicts postcondition
}

// SHOULD FAIL: Same-length unequal sequences, assert concat equal
proof fn mutation_test_2_same_length_assert_concat_equal() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 4];
    let suffix: Seq<int> = seq![5, 6];
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix); // SHOULD FAIL: postcondition says !=
}

// SHOULD FAIL: Different-length unequal sequences, assert concat equal
proof fn mutation_test_3_different_length_assert_concat_equal() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1];
    let suffix: Seq<int> = seq![3];
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix); // SHOULD FAIL: different lengths after concat
}

// SHOULD FAIL: Assert s1 == s2 after valid call — contradicts the known precondition
proof fn mutation_test_4_assert_inputs_equal() {
    let s1: Seq<int> = seq![10, 0];
    let s2: Seq<int> = seq![20, 0];
    let suffix: Seq<int> = seq![30];
    assert(s1[0] != s2[0]); // hint: prove s1 != s2
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1 == s2); // SHOULD FAIL: we know s1 != s2
}

}
