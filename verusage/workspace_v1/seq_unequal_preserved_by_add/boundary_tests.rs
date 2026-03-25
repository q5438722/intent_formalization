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

// ===== BOUNDARY TESTS =====
// These tests violate preconditions or use edge-case inputs.
// All should FAIL verification because the precondition s1 != s2 is not met.

// SHOULD FAIL: Equal concrete sequences violate the precondition s1 != s2
proof fn boundary_test_1_equal_sequences() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 3];
    let suffix: Seq<int> = seq![4];
    seq_unequal_preserved_by_add(s1, s2, suffix); // SHOULD FAIL: s1 == s2
}

// SHOULD FAIL: Both empty sequences are equal, violating the precondition
proof fn boundary_test_2_both_empty() {
    let s1: Seq<int> = Seq::<int>::empty();
    let s2: Seq<int> = Seq::<int>::empty();
    let suffix: Seq<int> = seq![1, 2];
    seq_unequal_preserved_by_add(s1, s2, suffix); // SHOULD FAIL: s1 == s2
}

// SHOULD FAIL: Using the same variable for both arguments violates s1 != s2
proof fn boundary_test_3_same_variable() {
    let s: Seq<int> = seq![42];
    let suffix: Seq<int> = Seq::<int>::empty();
    seq_unequal_preserved_by_add(s, s, suffix); // SHOULD FAIL: s == s
}

// SHOULD FAIL: Equal single-element sequences with empty suffix
proof fn boundary_test_4_equal_singleton_empty_suffix() {
    let s1: Seq<int> = seq![0];
    let s2: Seq<int> = seq![0];
    let suffix: Seq<int> = Seq::<int>::empty();
    seq_unequal_preserved_by_add(s1, s2, suffix); // SHOULD FAIL: s1 == s2
}

}
