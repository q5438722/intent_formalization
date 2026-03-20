// Behavioral Inconsistency Tests
//
// Query type: Behavioral (φ encodes input-output relations)
//
// These tests systematically mutate the expected output of bitmap operations and
// check whether the specification correctly REJECTS the mutated behavior. Each test
// constructs a concrete scenario, performs a bitmap operation, then asserts an
// INCORRECT output relationship. If the spec is consistent, all these tests should
// FAIL verification (i.e., be REJECTED by the verifier).
//
// Methodology: For each operation (new, set, clear, alloc, alloc_range, test),
// we generate adversarial assertions by:
//   (a) Negating the correct postcondition (e.g., assert bit NOT set after set())
//   (b) Substituting a wrong value (e.g., wrong index, wrong usage count)
//   (c) Violating frame conditions (e.g., asserting unrelated bits changed)
//
// Expected result: ALL tests should be REJECTED (verification failure).
// If any test PASSES, it indicates a behavioral inconsistency in the specification.

verus! {

//==================================================================================================
// Category 1: Output Negation — Negate the direct postcondition of an operation
//==================================================================================================

/// BEH01: After set(0), assert the bit is NOT set (negated postcondition).
fn test_beh01_set_negated_postcondition() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: set(0) guarantees is_bit_set(0) == true.
            assert(!bitmap@.is_bit_set(0));
        }
    }
}

/// BEH02: After clear(0), assert the bit is still set (negated postcondition).
fn test_beh02_clear_negated_postcondition() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // WRONG: clear(0) guarantees is_bit_set(0) == false.
                assert(bitmap@.is_bit_set(0));
            }
        }
    }
}

/// BEH03: After alloc(), assert the returned index's bit is NOT set.
fn test_beh03_alloc_bit_not_set() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(index) = a {
            // WRONG: alloc() returns an index that IS set.
            assert(!bitmap@.is_bit_set(index as int));
        }
    }
}

/// BEH04: After alloc_range(8), assert not all bits in range are set.
fn test_beh04_alloc_range_not_all_set() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc_range(8);
        if let Ok(start) = a {
            // WRONG: alloc_range guarantees all bits in [start, start+size) are set.
            assert(!bitmap@.all_bits_set_in_range(start as int, (start + 8) as int));
        }
    }
}

/// BEH05: After test(0) on empty bitmap, assert it returns true.
fn test_beh05_test_empty_returns_true() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let t = bitmap.test(0);
        if let Ok(b) = t {
            // WRONG: empty bitmap has no bits set, test(0) should return false.
            assert(b == true);
        }
    }
}

//==================================================================================================
// Category 2: Value Substitution — Assert a wrong value for a correct property
//==================================================================================================

/// BEH06: After set(0), assert usage is 2 (should be 1).
fn test_beh06_set_wrong_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: usage should be 1 after setting one bit.
            assert(bitmap@.usage() == 2);
        }
    }
}

/// BEH07: After alloc_range(4), assert usage increased by 3 (should be 4).
fn test_beh07_alloc_range_wrong_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc_range(4);
        if let Ok(_) = a {
            // WRONG: usage should be 4 after allocating range of size 4.
            assert(bitmap@.usage() == 3);
        }
    }
}

/// BEH08: After set(0) then set(1), assert usage is 1 (should be 2).
fn test_beh08_double_set_wrong_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                // WRONG: two successful sets => usage == 2.
                assert(bitmap@.usage() == 1);
            }
        }
    }
}

/// BEH09: After set(0); set(1); clear(0), assert usage is 0 (should be 1).
fn test_beh09_compound_wrong_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                let c = bitmap.clear(0);
                if let Ok(()) = c {
                    // WRONG: set(0); set(1); clear(0) => usage == 1.
                    assert(bitmap@.usage() == 0);
                }
            }
        }
    }
}

/// BEH10: After new(64), assert num_bits is 32 (should be 64).
fn test_beh10_new_wrong_num_bits() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        // WRONG: num_bits should equal the constructor argument.
        assert(bitmap@.num_bits == 32);
    }
}

//==================================================================================================
// Category 3: Frame Violation — Assert that unrelated state was modified
//==================================================================================================

/// BEH11: After set(0), assert bit 1 is also set (frame violation).
fn test_beh11_set_frame_violation() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: set(0) should not modify bit 1.
            assert(bitmap@.is_bit_set(1));
        }
    }
}

/// BEH12: After clear(0), assert bit 1 changed (frame violation).
fn test_beh12_clear_frame_violation() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(1);
            if let Ok(()) = s2 {
                let c = bitmap.clear(0);
                if let Ok(()) = c {
                    // WRONG: clear(0) should not affect bit 1.
                    assert(!bitmap@.is_bit_set(1));
                }
            }
        }
    }
}

/// BEH13: After alloc(), assert that num_bits changed (frame violation).
fn test_beh13_alloc_changes_num_bits() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(_) = a {
            // WRONG: alloc should not modify num_bits.
            assert(bitmap@.num_bits != 64);
        }
    }
}

/// BEH14: After alloc_range(4), assert a bit outside range was set (frame violation).
fn test_beh14_alloc_range_frame_violation() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc_range(4);
        if let Ok(start) = a {
            // Pick an index outside the allocated range.
            if start + 4 < 64 {
                // WRONG: bits outside [start, start+4) should remain unset on empty bitmap.
                assert(bitmap@.is_bit_set((start + 4) as int));
            }
        }
    }
}

/// BEH15: Two consecutive allocs return the same index (mutation of non-overlap).
fn test_beh15_two_allocs_same_index() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(i1) = a1 {
            let a2 = bitmap.alloc();
            if let Ok(i2) = a2 {
                // WRONG: two allocs on a fresh bitmap must return different indices.
                assert(i1 == i2);
            }
        }
    }
}

//==================================================================================================
// Category 4: Error Path Mutation — Assert error path behaves like success
//==================================================================================================

/// BEH16: Attempt to clear an already-clear bit and assert it succeeds.
fn test_beh16_clear_unset_succeeds() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Bit 0 is unset in a new bitmap.
        let c = bitmap.clear(0);
        // WRONG: clearing an already-unset bit should return Err.
        assert(c is Ok);
    }
}

/// BEH17: Attempt to set an already-set bit and assert it succeeds.
fn test_beh17_set_already_set_succeeds() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(0);
            // WRONG: setting an already-set bit should return Err.
            assert(s2 is Ok);
        }
    }
}

/// BEH18: After a failed operation (Err), assert state was mutated.
fn test_beh18_error_mutates_state() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ghost old_view = bitmap@;
        let c = bitmap.clear(0);
        if let Err(_) = c {
            // WRONG: failed clear should not change the bitmap.
            assert(bitmap@.usage() != old_view.usage());
        }
    }
}

} // verus!
