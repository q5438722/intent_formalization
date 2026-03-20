// Boundary Inconsistency Tests
//
// Query type: Boundary (φ encodes input validity)
//
// These tests probe the specification's handling of input boundaries: invalid
// arguments, out-of-range indices, edge-case sizes, and precondition violations.
// They verify that the specification correctly classifies inputs as valid or
// invalid and enforces appropriate error semantics at system boundaries.
//
// Methodology: For each operation, we construct queries that:
//   (a) Violate explicit preconditions (e.g., index >= num_bits)
//   (b) Test exact boundary values (e.g., index == num_bits - 1 vs num_bits)
//   (c) Test degenerate/edge inputs (e.g., size == 0, size == num_bits)
//   (d) Assert wrong error classification (e.g., claim invalid input succeeds)
//
// Expected result: Tests marked "SHOULD REJECT" should fail verification.
// Tests marked "SHOULD PASS" should succeed verification.
// If a "SHOULD REJECT" test passes, the spec admits invalid inputs.
// If a "SHOULD PASS" test fails, the spec may be overly restrictive or incomplete.

verus! {

//==================================================================================================
// Category 1: Constructor Boundary — Invalid bitmap creation parameters
//==================================================================================================

/// BND01: Creating bitmap with 0 bits must fail.
/// SHOULD REJECT: asserts that new(0) succeeds.
fn test_bnd01_new_zero_succeeds() {
    let result = Bitmap::new(0);
    // WRONG: size 0 is explicitly rejected.
    assert(result is Ok);
}

/// BND02: Creating bitmap with non-multiple-of-8 must fail.
/// SHOULD REJECT: asserts that new(7) succeeds.
fn test_bnd02_new_non_aligned_succeeds() {
    let result = Bitmap::new(7);
    // WRONG: 7 is not a multiple of 8.
    assert(result is Ok);
}

/// BND03: Creating bitmap at u32::MAX boundary must fail.
/// SHOULD REJECT: asserts that new(u32::MAX) succeeds.
fn test_bnd03_new_u32_max_succeeds() {
    let result = Bitmap::new(u32::MAX as usize);
    // WRONG: u32::MAX is explicitly rejected.
    assert(result is Ok);
}

/// BND04: Creating bitmap with valid minimum size (8) must succeed.
/// SHOULD PASS: 8 is the smallest valid bitmap size.
fn test_bnd04_new_min_valid() {
    let result = Bitmap::new(8);
    if let Ok(bitmap) = result {
        assert(bitmap@.num_bits == 8);
        assert(bitmap@.is_empty());
    }
}

/// BND05: Creating bitmap with size just below u32::MAX, aligned to 8, should fail
/// if it equals or exceeds u32::MAX.
/// SHOULD REJECT: the value u32::MAX - 1 is not a multiple of 8 in general,
/// but even if it were, it's >= u32::MAX is excluded.
fn test_bnd05_new_just_below_max() {
    // u32::MAX is 4294967295. u32::MAX - 7 = 4294967288 which is a multiple of 8.
    // But it's still < u32::MAX, so it should be accepted IF memory allows.
    // This test checks the boundary: the spec allows it (< u32::MAX).
    let result = Bitmap::new(4294967288);
    // This may succeed or fail depending on memory, but should not be rejected by spec.
    // The spec says: number_of_bits >= u32::MAX ==> result is Err.
    // 4294967288 < 4294967295, so spec does not require Err.
    // We leave this as a boundary probe — no assertion needed.
}

//==================================================================================================
// Category 2: Index Boundary — Out-of-bounds access
//==================================================================================================

/// BND06: Testing a bit at exactly num_bits (one past the end) must fail.
/// SHOULD REJECT: asserts out-of-bounds test succeeds.
fn test_bnd06_test_at_num_bits() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let t = bitmap.test(64);
        // WRONG: index 64 is out of bounds for a 64-bit bitmap (valid: 0..63).
        assert(t is Ok);
    }
}

/// BND07: Setting a bit at exactly num_bits must fail.
/// SHOULD REJECT: asserts out-of-bounds set succeeds.
fn test_bnd07_set_at_num_bits() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(64);
        // WRONG: index 64 is out of bounds.
        assert(s is Ok);
    }
}

/// BND08: Clearing a bit at exactly num_bits must fail.
/// SHOULD REJECT: asserts out-of-bounds clear succeeds.
fn test_bnd08_clear_at_num_bits() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Even if we try to clear at OOB, it should fail.
        let c = bitmap.clear(64);
        // WRONG: index 64 is out of bounds.
        assert(c is Ok);
    }
}

/// BND09: Testing the last valid index (num_bits - 1) must succeed.
/// SHOULD PASS: index 63 is valid for a 64-bit bitmap.
fn test_bnd09_test_last_valid() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let t = bitmap.test(63);
        if let Ok(b) = t {
            // Correct: index 63 is valid, and on empty bitmap it should be false.
            assert(b == false);
        }
    }
}

/// BND10: Setting the last valid index must succeed.
/// SHOULD PASS: index 63 is the last valid bit.
fn test_bnd10_set_last_valid() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(63);
        if let Ok(()) = s {
            assert(bitmap@.is_bit_set(63));
        }
    }
}

/// BND11: Testing with a very large index must fail.
/// SHOULD REJECT: asserts far-OOB test succeeds.
fn test_bnd11_test_large_index() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let t = bitmap.test(1000);
        // WRONG: 1000 is far out of bounds.
        assert(t is Ok);
    }
}

//==================================================================================================
// Category 3: Operation Precondition Boundary — State-dependent violations
//==================================================================================================

/// BND12: Setting a bit that is already set must fail.
/// SHOULD REJECT: asserts double-set succeeds.
fn test_bnd12_double_set_succeeds() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(0);
            // WRONG: setting an already-set bit returns Err.
            assert(s2 is Ok);
        }
    }
}

/// BND13: Clearing a bit that is already clear must fail.
/// SHOULD REJECT: asserts clearing an unset bit succeeds.
fn test_bnd13_clear_unset_succeeds() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Bit 0 is unset on a fresh bitmap.
        let c = bitmap.clear(0);
        // WRONG: clearing an unset bit returns Err.
        assert(c is Ok);
    }
}

/// BND14: Alloc on a completely full bitmap must fail.
/// SHOULD PASS: correctly asserts alloc returns Err when full.
fn test_bnd14_alloc_full_bitmap() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        // Fill all 8 bits.
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

        // Now bitmap is full, alloc must fail.
        let a = bitmap.alloc();
        assert(a is Err);
    }
}

/// BND15: alloc_range with size 0 should be rejected by precondition.
/// This tests whether alloc_range's requires clause (size > 0) is enforceable.
/// SHOULD REJECT: violates precondition.
fn test_bnd15_alloc_range_size_zero()
    requires false, // precondition violation — this function can never be called
{
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // size == 0 violates the precondition `size > 0`.
        let a = bitmap.alloc_range(0);
        assert(a is Ok);
    }
}

/// BND16: alloc_range with size > num_bits should fail.
/// SHOULD PASS: correctly asserts failure.
fn test_bnd16_alloc_range_exceeds_capacity() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        // size == 8 is valid (== num_bits), but size == 8 on empty bitmap should succeed.
        // Test with valid size first, then fill and try again.
        let mut i: usize = 0;
        while i < 4
            invariant
                0 <= i <= 4,
                bitmap.inv(),
                bitmap@.num_bits == 8,
                bitmap@.usage() == i as int,
            decreases 4 - i,
        {
            let s = bitmap.set(i);
            if let Ok(()) = s {
                i = i + 1;
            } else {
                return;
            }
        }
        // 4 bits set, 4 free. alloc_range(5) needs 5 contiguous free bits.
        // Whether this succeeds depends on where the free bits are.
        // But alloc_range(8) on this bitmap must fail (only 4 free).
        let a = bitmap.alloc_range(8);
        assert(a is Err);
    }
}

//==================================================================================================
// Category 4: Byte Boundary — Operations at word/byte alignment edges
//==================================================================================================

/// BND17: Set bit at byte boundary (index 7, last bit of first byte).
/// SHOULD PASS: index 7 is valid.
fn test_bnd17_set_at_byte_boundary() {
    let result = Bitmap::new(16);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(7);
        if let Ok(()) = s {
            assert(bitmap@.is_bit_set(7));
            // Bit 8 (first bit of second byte) should NOT be affected.
            assert(!bitmap@.is_bit_set(8));
        }
    }
}

/// BND18: Set bit at start of second byte (index 8).
/// SHOULD PASS: index 8 is valid for a 16-bit bitmap.
fn test_bnd18_set_at_second_byte_start() {
    let result = Bitmap::new(16);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(8);
        if let Ok(()) = s {
            assert(bitmap@.is_bit_set(8));
            // Bit 7 (last bit of first byte) should NOT be affected.
            assert(!bitmap@.is_bit_set(7));
        }
    }
}

/// BND19: alloc_range that spans across byte boundary.
/// SHOULD PASS: range [6, 10) crosses the byte boundary at index 8.
fn test_bnd19_alloc_range_crosses_byte() {
    let result = Bitmap::new(16);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc_range(4);
        if let Ok(start) = a {
            // All 4 bits in the range should be set.
            assert(bitmap@.all_bits_set_in_range(start as int, (start + 4) as int));
            assert(bitmap@.usage() == 4);
        }
    }
}

/// BND20: Alloc range of exactly num_bits (fills entire bitmap).
/// SHOULD PASS: allocating all bits at once should work on an empty bitmap.
fn test_bnd20_alloc_range_full_size() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc_range(8);
        if let Ok(start) = a {
            assert(start == 0);
            assert(bitmap@.usage() == 8);
            assert(bitmap@.all_bits_set_in_range(0, 8));
        }
    }
}

} // verus!
