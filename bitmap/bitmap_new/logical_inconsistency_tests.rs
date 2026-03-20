// Logical Inconsistency Tests
//
// Query type: Logical (φ encodes arbitrary semantic claims)
//
// These tests probe whether the specification entails (or correctly refuses to
// entail) higher-level semantic properties that go beyond simple input-output
// relations. They target properties such as:
//   - State invertibility (set then clear restores original state)
//   - Commutativity (order independence of operations)
//   - Monotonicity (usage only changes in expected direction)
//   - Mutual exclusion (empty ↔ ¬full)
//   - Idempotence and absorption
//   - Liveness guarantees (space available ⟹ operation succeeds)
//   - Determinism (whether the spec constrains operation outcomes uniquely)
//
// These queries detect a class of errors that behavioral mutation cannot:
// "logical inconsistencies" where the specification admits unintended reasoning
// even when all concrete behaviors appear correct.
//
// Expected results:
//   Tests marked "SHOULD PASS" assert correct semantic properties.
//     If they FAIL, the spec has a completeness gap.
//   Tests marked "SHOULD REJECT" assert incorrect semantic claims.
//     If they PASS, the spec is logically inconsistent (over-admits).

verus! {

//==================================================================================================
// Category 1: Invertibility — Operations that should cancel each other
//==================================================================================================

/// LOG01: set(i) then clear(i) should restore the set_bits to original.
/// SHOULD PASS: set followed by clear on the same index is an identity on set_bits.
fn test_log01_set_clear_restores_set_bits() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ghost orig_set_bits = bitmap@.set_bits;
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // set_bits should be restored to original (empty set).
                assert(bitmap@.set_bits =~= orig_set_bits);
            }
        }
    }
}

/// LOG02: set(i) then clear(i) should restore usage to original.
/// SHOULD PASS: usage should return to 0.
fn test_log02_set_clear_restores_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ghost orig_usage = bitmap@.usage();
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                assert(bitmap@.usage() == orig_usage);
            }
        }
    }
}

/// LOG03: set(i) then clear(i) should yield is_empty() again.
/// SHOULD PASS (may require extensional equality trigger).
fn test_log03_set_clear_is_empty() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // Trigger extensional equality.
                assert(bitmap@.set_bits =~= Set::<int>::empty());
                assert(bitmap@.is_empty());
            }
        }
    }
}

/// LOG04: Alloc then clear should restore to a state where the bit is free.
/// SHOULD PASS: after allocating and clearing, the bit should be available.
fn test_log04_alloc_clear_frees_bit() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(index) = a {
            let c = bitmap.clear(index);
            if let Ok(()) = c {
                assert(!bitmap@.is_bit_set(index as int));
                assert(bitmap@.usage() == 0);
            }
        }
    }
}

//==================================================================================================
// Category 2: Commutativity — Order independence of operations
//==================================================================================================

/// LOG05: set(0) then set(1) should produce the same set_bits as set(1) then set(0).
/// SHOULD PASS: set_bits = {0, 1} regardless of order.
fn test_log05_set_commutativity_path_a() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                // Both bits should be set.
                assert(bitmap@.is_bit_set(0));
                assert(bitmap@.is_bit_set(1));
                assert(bitmap@.usage() == 2);
                // set_bits should be {0, 1}.
                assert(bitmap@.set_bits.contains(0));
                assert(bitmap@.set_bits.contains(1));
            }
        }
    }
}

/// LOG06: set(1) then set(0) should produce the same result.
/// SHOULD PASS: same final state as LOG05.
fn test_log06_set_commutativity_path_b() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(1);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(0);
            if let Ok(()) = s2 {
                assert(bitmap@.is_bit_set(0));
                assert(bitmap@.is_bit_set(1));
                assert(bitmap@.usage() == 2);
                assert(bitmap@.set_bits.contains(0));
                assert(bitmap@.set_bits.contains(1));
            }
        }
    }
}

/// LOG07: clear(0) then clear(1) vs clear(1) then clear(0) — same final state.
/// SHOULD PASS: order of clears is commutative on set_bits.
fn test_log07_clear_commutativity() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                let s3 = bitmap.set(2);
                if let Ok(()) = s3 {
                    // Clear 0 then 1.
                    let c1 = bitmap.clear(0);
                    if let Ok(()) = c1 {
                        let c2 = bitmap.clear(1);
                        if let Ok(()) = c2 {
                            // Only bit 2 should remain.
                            assert(bitmap@.is_bit_set(2));
                            assert(!bitmap@.is_bit_set(0));
                            assert(!bitmap@.is_bit_set(1));
                            assert(bitmap@.usage() == 1);
                        }
                    }
                }
            }
        }
    }
}

//==================================================================================================
// Category 3: Mutual Exclusion — Properties that should be mutually exclusive
//==================================================================================================

/// LOG08: An empty bitmap should not be full (mutual exclusion for num_bits > 0).
/// SHOULD PASS: this is a key logical property (known gap in some spec versions).
fn test_log08_empty_not_full() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        // Need lemma to bridge: usage == 0 < 64 ⟹ ¬is_full.
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
        }
        assert(!bitmap@.is_full());
    }
}

/// LOG09: A full bitmap should not be empty.
/// SHOULD PASS: if all bits are set, set_bits ≠ ∅.
fn test_log09_full_not_empty() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let mut i: usize = 0;
        while i < 8
            invariant
                0 <= i <= 8,
                bitmap.inv(),
                bitmap@.num_bits == 8,
                bitmap@.usage() == i as int,
                forall|j: int| 0 <= j < i as int ==> bitmap@.is_bit_set(j),
            decreases 8 - i,
        {
            let s = bitmap.set(i);
            if let Ok(()) = s {
                i = i + 1;
            } else {
                return;
            }
        }
        // All bits set.
        assert(bitmap@.usage() == 8);
        // Full should be true.
        proof {
            bitmap@.lemma_usage_equals_number_of_bits_implies_full();
        }
        assert(bitmap@.is_full());
        // Should NOT be empty.
        assert(!bitmap@.is_empty());
    }
}

/// LOG10: Incorrectly claim a bitmap is both empty and has a set bit.
/// SHOULD REJECT: is_empty() ∧ is_bit_set(i) is a contradiction.
fn test_log10_empty_and_set_contradiction() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: bitmap has bit 0 set, so it cannot be empty.
            assert(bitmap@.is_empty());
        }
    }
}

/// LOG11: Incorrectly claim usage > num_bits after any sequence of operations.
/// SHOULD REJECT: usage is bounded by num_bits (invariant).
fn test_log11_usage_exceeds_capacity() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(_) = a {
            // WRONG: usage can never exceed num_bits by invariant.
            assert(bitmap@.usage() > bitmap@.num_bits);
        }
    }
}

//==================================================================================================
// Category 4: Monotonicity — Directional properties of operations
//==================================================================================================

/// LOG12: Usage monotonically increases with set/alloc.
/// SHOULD PASS: each successful set increases usage by exactly 1.
fn test_log12_set_increases_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ghost u0 = bitmap@.usage();
        let s = bitmap.set(0);
        if let Ok(()) = s {
            assert(bitmap@.usage() == u0 + 1);
            let ghost u1 = bitmap@.usage();
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                assert(bitmap@.usage() == u1 + 1);
            }
        }
    }
}

/// LOG13: Usage monotonically decreases with clear.
/// SHOULD PASS: each successful clear decreases usage by exactly 1.
fn test_log13_clear_decreases_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                let ghost u_before = bitmap@.usage();
                let c = bitmap.clear(0);
                if let Ok(()) = c {
                    assert(bitmap@.usage() == u_before - 1);
                }
            }
        }
    }
}

/// LOG14: Incorrectly claim usage decreases after a successful set.
/// SHOULD REJECT: set always increases usage.
fn test_log14_set_decreases_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ghost u0 = bitmap@.usage();
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: set should increase, not decrease, usage.
            assert(bitmap@.usage() < u0);
        }
    }
}

/// LOG15: Incorrectly claim usage increases after a successful clear.
/// SHOULD REJECT: clear always decreases usage.
fn test_log15_clear_increases_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let ghost u_before = bitmap@.usage();
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // WRONG: clear should decrease, not increase, usage.
                assert(bitmap@.usage() > u_before);
            }
        }
    }
}

//==================================================================================================
// Category 5: Liveness — Operations that must succeed under sufficient conditions
//==================================================================================================

/// LOG16: Alloc on an empty bitmap must succeed (has free bits ⟹ liveness).
/// SHOULD PASS: empty bitmap has usage 0 < num_bits, so a free bit exists.
fn test_log16_alloc_liveness_empty() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
        }
        let a = bitmap.alloc();
        assert(a is Ok);
    }
}

/// LOG17: After clearing a bit, alloc must succeed (space available ⟹ liveness).
/// SHOULD PASS: clearing creates a free bit, so alloc should find it.
fn test_log17_alloc_after_clear_succeeds() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        // Fill bitmap.
        let mut i: usize = 0;
        while i < 8
            invariant
                0 <= i <= 8,
                bitmap.inv(),
                bitmap@.num_bits == 8,
                bitmap@.usage() == i as int,
                forall|j: int| 0 <= j < i as int ==> bitmap@.is_bit_set(j),
            decreases 8 - i,
        {
            let s = bitmap.set(i);
            if let Ok(()) = s {
                i = i + 1;
            } else {
                return;
            }
        }
        // Clear one bit to make space.
        let c = bitmap.clear(3);
        if let Ok(()) = c {
            // Now there's a free bit, alloc must succeed.
            proof {
                bitmap@.lemma_unset_bit_implies_has_free_bit(3);
                bitmap@.lemma_has_free_bit_implies_exists_free_range_1();
            }
            let a = bitmap.alloc();
            assert(a is Ok);
        }
    }
}

/// LOG18: alloc_range should succeed when enough contiguous free space exists.
/// SHOULD PASS: empty 64-bit bitmap has contiguous range of 8 free bits.
fn test_log18_alloc_range_liveness() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
        }
        let a = bitmap.alloc_range(8);
        assert(a is Ok);
    }
}

//==================================================================================================
// Category 6: Stronger Properties — Claims the spec should NOT entail
//==================================================================================================

/// LOG19: Incorrectly claim alloc always returns index 0.
/// SHOULD REJECT: alloc may return any free index; the spec does not
/// guarantee deterministic index selection.
fn test_log19_alloc_determinism_false() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Set bit 0 so it's not available.
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let a = bitmap.alloc();
            if let Ok(index) = a {
                // WRONG: the spec does not require alloc to return 0
                // (and it can't, since bit 0 is already set).
                assert(index == 0);
            }
        }
    }
}

/// LOG20: Incorrectly claim that two alloc_range calls of the same size
/// must produce adjacent ranges.
/// SHOULD REJECT: the spec guarantees non-overlap, not adjacency.
fn test_log20_alloc_range_adjacency_false() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc_range(8);
        if let Ok(s1) = a1 {
            let a2 = bitmap.alloc_range(8);
            if let Ok(s2) = a2 {
                // WRONG: spec guarantees non-overlap but NOT adjacency.
                // s2 == s1 + 8 may happen to be true for this impl, but the
                // spec does not guarantee it.
                // Actually, let's test a stronger false claim: s2 == s1.
                assert(s2 == s1);
            }
        }
    }
}

/// LOG21: Incorrectly claim set_bits equality implies structural equality.
/// SHOULD REJECT: set_bits being the same set does not imply bitmap views
/// are identical (they could differ in num_bits).
fn test_log21_set_bits_eq_implies_view_eq() {
    let r1 = Bitmap::new(64);
    let r2 = Bitmap::new(128);
    if let Ok(b1) = r1 {
        if let Ok(b2) = r2 {
            // Both empty, so set_bits are both empty sets.
            assert(b1@.set_bits =~= b2@.set_bits);
            // WRONG: views are not equal because num_bits differ.
            assert(b1@.num_bits == b2@.num_bits);
        }
    }
}

/// LOG22: Incorrectly claim alloc_range(1) and alloc() always return the same index
/// on identical initial states.
/// SHOULD REJECT: while they may behave similarly, the spec does not
/// require identical index selection.
fn test_log22_alloc_vs_alloc_range_same_result() {
    let r1 = Bitmap::new(64);
    let r2 = Bitmap::new(64);
    if let Ok(mut b1) = r1 {
        if let Ok(mut b2) = r2 {
            // Set bit 5 in both.
            let s1 = b1.set(5);
            let s2 = b2.set(5);
            if let Ok(()) = s1 {
                if let Ok(()) = s2 {
                    let a1 = b1.alloc();
                    let a2 = b2.alloc_range(1);
                    if let Ok(i1) = a1 {
                        if let Ok(i2) = a2 {
                            // The spec does not require these to be the same index.
                            // (Though they might be in practice.)
                            // We test a FALSE claim: they must differ.
                            assert(i1 != i2);
                        }
                    }
                }
            }
        }
    }
}

} // verus!
