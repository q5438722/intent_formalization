// Consistency-Checking Proof Tests for BitmapView Specification
//
// These are adversarial queries: if the specification is correct,
// each test should FAIL to verify.

verus! {

// ============================================================
// BOUNDARY CONSISTENCY TESTS (Precondition Violations)
// ============================================================

// --- Test 1: lemma_range_set_finite with lo > hi ---
// Requires: lo <= hi. Here lo=5, hi=3 violates this.
// SHOULD FAIL
proof fn test_boundary_range_set_finite_lo_gt_hi() {
    BitmapView::lemma_range_set_finite(5, 3);
}

// --- Test 2: lemma_range_set_len with lo > hi ---
// Requires: lo <= hi. Here lo=10, hi=5 violates this.
// SHOULD FAIL
proof fn test_boundary_range_set_len_lo_gt_hi() {
    BitmapView::lemma_range_set_len(10, 5);
}

// --- Test 3: lemma_free_range_implies_usage_bound with n == 0 ---
// Requires: n > 0. Here n=0 violates this.
// SHOULD FAIL
proof fn test_boundary_free_range_usage_bound_n_zero() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.has_free_range_at(0, 0));
    bv.lemma_free_range_implies_usage_bound(0, 0);
}

// --- Test 4: lemma_insert_preserves_usage_bound with x == num_bits (out of range) ---
// Requires: 0 <= x < num_bits. Here x=8 == num_bits violates this.
// SHOULD FAIL
proof fn test_boundary_insert_x_equals_num_bits() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    bv.lemma_insert_preserves_usage_bound(8);
}

// --- Test 5: lemma_insert_preserves_usage_bound with x negative ---
// Requires: 0 <= x < num_bits. Here x=-1 violates this.
// SHOULD FAIL
proof fn test_boundary_insert_x_negative() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    bv.lemma_insert_preserves_usage_bound(-1);
}

// --- Test 6: lemma_insert_preserves_usage_bound with x already in set_bits ---
// Requires: !set_bits.contains(x). Here x=3 is in set_bits.
// SHOULD FAIL
proof fn test_boundary_insert_x_already_in_set() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty().insert(3) };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    bv.lemma_insert_preserves_usage_bound(3);
}

// --- Test 7: lemma_usage_less_than_capacity with usage == num_bits ---
// Requires: usage() < num_bits. Here usage == num_bits violates this.
// SHOULD FAIL
proof fn test_boundary_usage_equals_capacity() {
    let bv = BitmapView { num_bits: 8, set_bits: BitmapView::range_set(0, 8) };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.usage() == 8);
    bv.lemma_usage_less_than_capacity_means_not_full();
}

// --- Test 8: lemma_unset_bit_implies_has_free_bit with i == num_bits ---
// Requires: 0 <= i < num_bits. Here i=8 is out of bounds.
// SHOULD FAIL
proof fn test_boundary_unset_bit_out_of_bounds() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    bv.lemma_unset_bit_implies_has_free_bit(8);
}

// --- Test 9: lemma_set_bits_finite with violated wf ---
// Requires: wf(). Here set_bits contains 100 which is >= num_bits=4.
// SHOULD FAIL
proof fn test_boundary_set_bits_finite_no_wf() {
    let bv = BitmapView { num_bits: 4, set_bits: Set::<int>::empty().insert(100) };
    bv.lemma_set_bits_finite();
}

// --- Test 10: lemma_unset_bit_implies_has_free_bit with negative index ---
// Requires: 0 <= i. Here i=-1 violates this.
// SHOULD FAIL
proof fn test_boundary_unset_bit_negative_index() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    bv.lemma_unset_bit_implies_has_free_bit(-1);
}

// ============================================================
// BEHAVIORAL CONSISTENCY TESTS (Mutation-style)
// ============================================================

// --- Test 11: range_set_len off-by-one (assert hi - lo + 1) ---
// Correct ensures: len == hi - lo. Assert len == hi - lo + 1.
// SHOULD FAIL
proof fn test_behavioral_range_set_len_off_by_one_plus() {
    BitmapView::lemma_range_set_len(0, 8);
    assert(BitmapView::range_set(0, 8).len() == 9);
}

// --- Test 12: range_set_len off-by-one (assert hi - lo - 1) ---
// Correct ensures: len == hi - lo. Assert len == hi - lo - 1.
// SHOULD FAIL
proof fn test_behavioral_range_set_len_off_by_one_minus() {
    BitmapView::lemma_range_set_len(0, 8);
    assert(BitmapView::range_set(0, 8).len() == 7);
}

// --- Test 13: free_range_implies_usage_bound – assert strictly stronger bound ---
// Scenario: 12 of 16 bits set, free range at [12,16) of size 4.
// Correct: usage <= num_bits - n = 12. Assert usage <= 11.
// SHOULD FAIL
proof fn test_behavioral_free_range_strictly_stronger_bound() {
    let s = BitmapView::range_set(0, 12);
    let bv = BitmapView { num_bits: 16, set_bits: s };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.usage() == 12);
    assume(bv.has_free_range_at(12, 4));
    bv.lemma_free_range_implies_usage_bound(12, 4);
    // Correct: usage() <= 16 - 4 = 12, which holds (12 <= 12).
    // Assert stronger: usage() <= 16 - 5 = 11, which is 12 <= 11, false.
    assert(bv.usage() <= bv.num_bits - 5);
}

// --- Test 14: insert_preserves_usage_bound – assert strict inequality ---
// Scenario: num_bits=1, empty set, insert(0). Result len = 1 = num_bits.
// Correct: len <= num_bits. Assert len < num_bits (strict).
// SHOULD FAIL
proof fn test_behavioral_insert_strict_inequality() {
    let bv = BitmapView { num_bits: 1, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    bv.lemma_insert_preserves_usage_bound(0);
    // len of {0} is 1, num_bits is 1. 1 < 1 is false.
    assert(bv.set_bits.insert(0).len() < bv.num_bits);
}

// --- Test 15: usage_equals_capacity_implies_full – assert a specific bit is NOT set ---
// After calling lemma, ensures all bits in [0, num_bits) are set.
// Assert one specific bit is NOT set.
// SHOULD FAIL
proof fn test_behavioral_full_but_one_bit_unset() {
    let bv = BitmapView { num_bits: 8, set_bits: BitmapView::range_set(0, 8) };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.usage() == 8);
    bv.lemma_usage_equals_number_of_bits_implies_full();
    // Lemma ensures: forall|i| 0<=i<8 ==> is_bit_set(i)
    // Assert bit 5 is NOT set (contradiction):
    assert(!bv.is_bit_set(5));
}

// --- Test 16: usage_less_than_capacity – assert bitmap IS full ---
// Correct: usage < num_bits implies !is_full(). Assert is_full().
// SHOULD FAIL
proof fn test_behavioral_underfull_but_assert_full() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.usage() == 0);
    assume(bv.usage() < bv.num_bits);
    bv.lemma_usage_less_than_capacity_means_not_full();
    assert(bv.is_full());
}

// --- Test 17: has_free_bit_implies_exists_free_range_1 – assert range of 2 ---
// Correct: has_free_bit ==> exists_contiguous_free_range(1).
// Assert exists_contiguous_free_range(2) – not guaranteed.
// SHOULD FAIL
proof fn test_behavioral_free_bit_implies_free_range_2() {
    // 7 out of 8 bits set; only bit 4 is free.
    // has_free_bit is true, but no contiguous range of size 2 exists.
    let s = BitmapView::range_set(0, 8).remove(4);
    let bv = BitmapView { num_bits: 8, set_bits: s };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.has_free_bit());
    bv.lemma_has_free_bit_implies_exists_free_range_1();
    // Lemma only guarantees range of 1; assert range of 2:
    assert(bv.exists_contiguous_free_range(2));
}

// ============================================================
// LOGICAL CONSISTENCY TESTS (R5-style)
// ============================================================

// --- Test 18: is_full implies is_empty (over-strong contradiction) ---
// A full bitmap cannot be empty (when num_bits > 0).
// SHOULD FAIL
proof fn test_logical_full_implies_empty() {
    let bv = BitmapView { num_bits: 8, set_bits: BitmapView::range_set(0, 8) };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.is_full());
    assert(bv.is_empty());
}

// --- Test 19: has_free_bit implies is_full (contradiction) ---
// Having a free bit means the bitmap is NOT full.
// SHOULD FAIL
proof fn test_logical_has_free_bit_implies_full() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.has_free_bit());
    assert(bv.is_full());
}

// --- Test 20: empty bitmap has zero num_bits (false structural assumption) ---
// An empty bitmap (no bits set) can have any positive num_bits.
// SHOULD FAIL
proof fn test_logical_empty_implies_zero_num_bits() {
    let bv = BitmapView { num_bits: 64, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.is_empty());
    // False conclusion: empty means num_bits == 0
    assert(bv.num_bits == 0);
}

// --- Test 21: two bitmaps with same usage have same set_bits (determinism assumption) ---
// Different sets can have the same cardinality.
// SHOULD FAIL
proof fn test_logical_same_usage_same_set_bits() {
    let bv1 = BitmapView { num_bits: 8, set_bits: Set::<int>::empty().insert(0) };
    let bv2 = BitmapView { num_bits: 8, set_bits: Set::<int>::empty().insert(7) };
    assume(bv1.wf());
    assume(bv2.wf());
    assume(bv1.set_bits.finite());
    assume(bv2.set_bits.finite());
    assume(bv1.usage() == bv2.usage());
    // Same usage does NOT imply same set_bits:
    assert(bv1.set_bits =~= bv2.set_bits);
}

// --- Test 22: range_set equality for shifted ranges (false structural assumption) ---
// range_set(0, n) != range_set(1, n+1) in general.
// SHOULD FAIL
proof fn test_logical_shifted_range_sets_equal() {
    BitmapView::lemma_range_set_len(0, 8);
    BitmapView::lemma_range_set_len(1, 9);
    // Both have length 8, but are different sets:
    assert(BitmapView::range_set(0, 8) =~= BitmapView::range_set(1, 9));
}

// --- Test 23: usage bound is exact (over-strong: usage == num_bits - n) ---
// The lemma only guarantees usage <= num_bits - n, not equality.
// SHOULD FAIL
proof fn test_logical_free_range_usage_exact() {
    let bv = BitmapView { num_bits: 16, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.usage() == 0);
    assume(bv.has_free_range_at(0, 4));
    bv.lemma_free_range_implies_usage_bound(0, 4);
    // Lemma ensures: usage <= num_bits - n = 12. Actual usage = 0.
    // Assert that usage is EXACTLY num_bits - n:
    assert(bv.usage() == bv.num_bits - 4);
}

// --- Test 24: wf implies all indices are set (over-strong) ---
// wf() only says set_bits ⊆ [0, num_bits). It does NOT say all indices are set.
// SHOULD FAIL
proof fn test_logical_wf_implies_all_set() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assert(bv.is_full());
}

// --- Test 25: exists_contiguous_free_range(n) implies has_free_range_at(0, n) ---
// The free range can start at any position, not necessarily 0.
// SHOULD FAIL
proof fn test_logical_free_range_starts_at_zero() {
    // 4 bits set at [0,4), free range is at [4,8)
    let bv = BitmapView { num_bits: 8, set_bits: BitmapView::range_set(0, 4) };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.exists_contiguous_free_range(4));
    // The free range is at [4,8), NOT at [0,4)
    assert(bv.has_free_range_at(0, 4));
}

// --- Test 26: set_bits_equal lemma implies same usage without equal num_bits ---
// The lemma requires both wf(), equal set_bits, AND equal num_bits.
// Here we violate equal num_bits but keep same set_bits.
// SHOULD FAIL
proof fn test_logical_equal_sets_different_num_bits_same_range() {
    let bv1 = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    let bv2 = BitmapView { num_bits: 16, set_bits: Set::<int>::empty() };
    assume(bv1.wf());
    assume(bv2.wf());
    // Same set_bits but different num_bits:
    bv1.lemma_set_bits_equal_has_free_range_at_equal(&bv2, 0, 4);
}

// --- Test 27: has_free_range_at with start + n > num_bits (misuse of spec) ---
// has_free_range_at requires start + n <= num_bits. Test a misuse scenario.
// SHOULD FAIL
proof fn test_logical_free_range_exceeds_capacity() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    // Assert free range of 5 starting at position 5 (5+5=10 > 8):
    assert(bv.has_free_range_at(5, 5));
}

// --- Test 28: is_empty and is_full simultaneously (contradiction) ---
// A bitmap with num_bits > 0 cannot be both empty and full.
// SHOULD FAIL
proof fn test_logical_empty_and_full_simultaneously() {
    let bv = BitmapView { num_bits: 8, set_bits: Set::<int>::empty() };
    assume(bv.wf());
    assume(bv.set_bits.finite());
    assume(bv.is_empty());
    assert(bv.is_full());
}

} // verus!
