// Bitmap Allocator - Generated Executable Verus Test Suite
//
// Tests for: new, from_raw_array, number_of_bits, alloc, alloc_range,
//            set, clear, test, index, index_unchecked.

verus! {

//==================================================================================================
// Constructor Tests
//==================================================================================================

/// Test: new with zero length should fail.
fn test_new_zero_length() {
    let result = Bitmap::new(0);
    assert(result is Err);
}

/// Test: new with length not a multiple of 8 should fail.
fn test_new_non_multiple_of_8() {
    let result = Bitmap::new(7);
    assert(result is Err);
}

/// Test: new with valid length should succeed and produce an empty bitmap.
fn test_new_valid() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        assert(bitmap.inv());
        assert(bitmap@.number_of_bits() == 64);
        assert(bitmap@.is_empty());
        assert(bitmap@.usage() == 0);
    }
}

/// Test: new bitmap has all bits unset.
fn test_new_all_bits_unset(number_of_bits: usize, idx: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        idx < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(bitmap) = result {
        assert(!bitmap.is_bit_set(idx as int));
    }
}

//==================================================================================================
// Getter Tests
//==================================================================================================

/// Test: number_of_bits returns the correct value.
fn test_number_of_bits_getter() {
    let result = Bitmap::new(128);
    if let Ok(bitmap) = result {
        let n = bitmap.number_of_bits();
        assert(n == 128);
    }
}

//==================================================================================================
// Set / Test / Clear Tests
//==================================================================================================

/// Test: setting a bit makes it test true, clearing makes it test false.
fn test_set_test_clear(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        // Initially unset.
        let t0 = bitmap.test(index);
        if let Ok(val) = t0 {
            assert(!val);
        }

        // Set the bit.
        let set_res = bitmap.set(index);
        if let Ok(()) = set_res {
            // Now test should return true.
            let t1 = bitmap.test(index);
            if let Ok(val) = t1 {
                assert(val);
            }

            // Clear the bit.
            let clr_res = bitmap.clear(index);
            if let Ok(()) = clr_res {
                // Now test should return false.
                let t2 = bitmap.test(index);
                if let Ok(val) = t2 {
                    assert(!val);
                }
            }
        }
    }
}

/// Test: setting an already-set bit should fail.
fn test_set_already_set(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let set1 = bitmap.set(index);
        if let Ok(()) = set1 {
            // Second set on same index should fail.
            let set2 = bitmap.set(index);
            assert(set2 is Err);
        }
    }
}

/// Test: clearing an already-cleared bit should fail.
fn test_clear_already_cleared(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        // Bit is initially unset, clearing should fail.
        let clr = bitmap.clear(index);
        assert(clr is Err);
    }
}

/// Test: test on an out-of-bounds index should fail.
fn test_test_out_of_bounds(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(bitmap) = result {
        let t = bitmap.test(number_of_bits);
        assert(t is Err);
    }
}

/// Test: set on an out-of-bounds index should fail.
fn test_set_out_of_bounds(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(number_of_bits);
        assert(s is Err);
    }
}

/// Test: setting a bit preserves other bits (frame property).
fn test_set_preserves_other_bits(number_of_bits: usize, i: usize, j: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        i < number_of_bits,
        j < number_of_bits,
        i != j,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(i);
        if let Ok(()) = s1 {
            // j should still be unset.
            assert(!bitmap.is_bit_set(j as int));
        }
    }
}

/// Test: clearing a bit preserves other bits (frame property).
fn test_clear_preserves_other_bits(number_of_bits: usize, i: usize, j: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        i < number_of_bits,
        j < number_of_bits,
        i != j,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        // Set both bits.
        let s1 = bitmap.set(i);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(j);
            if let Ok(()) = s2 {
                // Clear bit i.
                let clr = bitmap.clear(i);
                if let Ok(()) = clr {
                    // j should still be set.
                    assert(bitmap.is_bit_set(j as int));
                    // i should be unset.
                    assert(!bitmap.is_bit_set(i as int));
                }
            }
        }
    }
}

//==================================================================================================
// Alloc Tests
//==================================================================================================

/// Test: alloc on an empty bitmap should succeed.
fn test_alloc_on_empty(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let alloc_res = bitmap.alloc();
        if let Ok(index) = alloc_res {
            assert(index < number_of_bits);
            assert(bitmap.is_bit_set(index as int));
            assert(bitmap@.usage() == 1);
        }
    }
}

/// Test: two successive allocs return different indices.
fn test_two_allocs_different(number_of_bits: usize)
    requires
        number_of_bits >= 8,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(idx1) = a1 {
            let a2 = bitmap.alloc();
            if let Ok(idx2) = a2 {
                assert(idx1 != idx2);
                assert(bitmap.is_bit_set(idx1 as int));
                assert(bitmap.is_bit_set(idx2 as int));
                assert(bitmap@.usage() == 2);
            }
        }
    }
}

//==================================================================================================
// Alloc Range Tests
//==================================================================================================

/// Test: alloc_range with size 0 should fail.
fn test_alloc_range_size_zero() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(0);
        assert(ar is Err);
    }
}

/// Test: alloc_range with size > number_of_bits should fail.
fn test_alloc_range_size_too_large() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(65);
        assert(ar is Err);
    }
}

/// Test: alloc_range sets all bits in the returned range.
fn test_alloc_range_sets_bits(number_of_bits: usize, size: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        size > 0,
        size <= number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(size);
        if let Ok(start) = ar {
            assert(start + size <= number_of_bits);
            assert(bitmap.all_bits_set_in_range(start as int, (start + size) as int));
            assert(bitmap@.usage() == size as int);
        }
    }
}

/// Test: alloc_range preserves bits outside the allocated range.
fn test_alloc_range_frame(number_of_bits: usize, size: usize, other: usize)
    requires
        number_of_bits >= 8,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        size > 0,
        size < number_of_bits,
        other < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        // Set a bit first.
        let s = bitmap.set(other);
        if let Ok(()) = s {
            let ar = bitmap.alloc_range(size);
            if let Ok(start) = ar {
                // If other is outside the allocated range, it should still be set.
                if other < start || other >= start + size {
                    assert(bitmap.is_bit_set(other as int));
                }
            }
        }
    }
}

//==================================================================================================
// Usage Tracking Tests
//==================================================================================================

/// Test: usage increments on set, decrements on clear.
fn test_usage_tracking(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        assert(bitmap@.usage() == 0);

        let s = bitmap.set(index);
        if let Ok(()) = s {
            assert(bitmap@.usage() == 1);

            let c = bitmap.clear(index);
            if let Ok(()) = c {
                assert(bitmap@.usage() == 0);
            }
        }
    }
}

/// Test: alloc increases usage by 1.
fn test_alloc_usage(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(_) = a {
            assert(bitmap@.usage() == 1);
        }
    }
}

/// Test: alloc_range increases usage by size.
fn test_alloc_range_usage(number_of_bits: usize, size: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        size > 0,
        size <= number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(size);
        if let Ok(_) = ar {
            assert(bitmap@.usage() == size as int);
        }
    }
}

//==================================================================================================
// number_of_bits Constancy Tests
//==================================================================================================

/// Test: number_of_bits remains constant across set/clear/alloc operations.
fn test_number_of_bits_constant(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost nb = bitmap@.number_of_bits();

        let a = bitmap.alloc();
        if let Ok(_) = a {
            assert(bitmap@.number_of_bits() == nb);
        }

        let s = bitmap.set(index);
        match s {
            Ok(()) => { assert(bitmap@.number_of_bits() == nb); },
            Err(_) => { assert(bitmap@.number_of_bits() == nb); },
        }
    }
}

//==================================================================================================
// Index Tests
//==================================================================================================

/// Test: index on a valid index returns correct word/bit pair.
fn test_index_valid(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(bitmap) = result {
        let idx_res = bitmap.index(index);
        if let Ok((word, bit)) = idx_res {
            assert(word as int == index as int / (u8::BITS as int));
            assert(bit as int == index as int % (u8::BITS as int));
            assert(bit < u8::BITS as usize);
        }
    }
}

/// Test: index on an out-of-bounds index should fail.
fn test_index_out_of_bounds(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(bitmap) = result {
        let idx_res = bitmap.index(number_of_bits);
        assert(idx_res is Err);
    }
}

/// Test: index_unchecked returns correct word/bit pair.
fn test_index_unchecked_valid(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(bitmap) = result {
        let (word, bit) = bitmap.index_unchecked(index);
        assert(word as int == index as int / (u8::BITS as int));
        assert(bit as int == index as int % (u8::BITS as int));
        assert(bit < u8::BITS as usize);
    }
}

//==================================================================================================
// Alloc-Clear-Realloc Tests
//==================================================================================================

/// Test: clearing and re-allocating should reclaim the freed bit.
fn test_clear_and_realloc(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(idx1) = a1 {
            assert(bitmap.is_bit_set(idx1 as int));

            let clr = bitmap.clear(idx1);
            if let Ok(()) = clr {
                assert(!bitmap.is_bit_set(idx1 as int));
                assert(bitmap@.usage() == 0);

                // Re-allocate; should succeed since we freed a bit.
                let a2 = bitmap.alloc();
                if let Ok(idx2) = a2 {
                    assert(bitmap.is_bit_set(idx2 as int));
                    assert(bitmap@.usage() == 1);
                }
            }
        }
    }
}

//==================================================================================================
// Full Bitmap Tests
//==================================================================================================

/// Test: allocating on a full bitmap should fail.
fn test_alloc_full_bitmap() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        // Fill all 8 bits.
        let mut i: usize = 0;
        while i < 8
            invariant
                0 <= i <= 8,
                bitmap.inv(),
                bitmap@.number_of_bits() == 8,
                bitmap@.usage() == i as int,
            decreases 8 - i,
        {
            let a = bitmap.alloc();
            if let Ok(_) = a {
                i = i + 1;
            } else {
                break;
            }
        }

        if i == 8 {
            // Bitmap is full; next alloc should fail.
            let a = bitmap.alloc();
            assert(a is Err);
        }
    }
}

} // verus!
