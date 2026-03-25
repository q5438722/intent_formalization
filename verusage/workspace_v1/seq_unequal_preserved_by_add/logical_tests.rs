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

// ===== LOGICAL TESTS =====
// Test properties NOT explicitly guaranteed by the spec.
// These probe whether the spec allows unintended reasoning.

// SHOULD FAIL: The spec does NOT imply that unequal sequences must have different lengths.
// s1 != s2 does NOT entail s1.len() != s2.len() (they can differ in values only)
proof fn logical_test_1_length_inequality_not_implied() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 3];
    let suffix: Seq<int> = Seq::<int>::empty();
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1.len() != s2.len()); // SHOULD FAIL: same length, different values
}

// SHOULD FAIL: The spec does NOT imply that the first elements must differ.
// s1 != s2 does NOT entail s1[0] != s2[0]
proof fn logical_test_2_first_element_inequality_not_implied() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 3];
    let suffix: Seq<int> = seq![4];
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(s1[0] != s2[0]); // SHOULD FAIL: first elements are both 1
}

// SHOULD FAIL: The spec does NOT guarantee cross-suffix inequality.
// s1 != s2 with DIFFERENT suffixes does not imply s1 + suffix1 != s2 + suffix2
// Counterexample: s1=[1,2], s2=[1], suffix1=[], suffix2=[2] => s1+suffix1=[1,2] == s2+suffix2=[1,2]
proof fn logical_test_3_cross_suffix_inequality_not_implied() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1];
    let suffix1: Seq<int> = Seq::<int>::empty();
    let suffix2: Seq<int> = seq![2];
    assert(s1.len() != s2.len()); // hint: different lengths prove s1 != s2
    seq_unequal_preserved_by_add(s1, s2, suffix1);
    assert(s1 + suffix1 != s2 + suffix2); // SHOULD FAIL: [1,2] == [1,2]
}

// SHOULD FAIL: The spec does NOT imply that the suffix must be non-empty.
// s1 != s2 ==> s1 + suffix != s2 + suffix holds even for empty suffix,
// so suffix.len() > 0 is NOT a consequence of the spec.
proof fn logical_test_4_suffix_nonempty_not_implied() {
    let s1: Seq<int> = seq![1, 2];
    let s2: Seq<int> = seq![1, 3];
    let suffix: Seq<int> = Seq::<int>::empty();
    seq_unequal_preserved_by_add(s1, s2, suffix);
    assert(suffix.len() > 0); // SHOULD FAIL: suffix can be empty
}

}
