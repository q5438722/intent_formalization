// Bitmap Allocator - Generated Executable Verus Test Suite
//
// [CORRECTNESS] Tests for: new, from_raw_array, number_of_bits, alloc, alloc_range,
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

/// Test: new with u32::MAX should fail.
fn test_new_u32_max() {
    let result = Bitmap::new(u32::MAX as usize);
    assert(result is Err);
}

/// Test: new with valid length should succeed and produce an empty bitmap.
fn test_new_valid() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        assert(bitmap.inv());
        assert(bitmap@.num_bits == 64);
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
        assert(!bitmap@.is_bit_set(idx as int));
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
            assert(!bitmap@.is_bit_set(j as int));
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
                    assert(bitmap@.is_bit_set(j as int));
                    // i should be unset.
                    assert(!bitmap@.is_bit_set(i as int));
                }
            }
        }
    }
}

/// Test: set updates the set_bits abstraction correctly.
fn test_set_updates_set_bits(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost old_set_bits = bitmap@.set_bits;
        let s = bitmap.set(index);
        if let Ok(()) = s {
            assert(bitmap@.set_bits =~= old_set_bits.insert(index as int));
        }
    }
}

/// Test: clear updates the set_bits abstraction correctly.
fn test_clear_updates_set_bits(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(index);
        if let Ok(()) = s {
            let ghost old_set_bits = bitmap@.set_bits;
            let c = bitmap.clear(index);
            if let Ok(()) = c {
                assert(bitmap@.set_bits =~= old_set_bits.remove(index as int));
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
            assert(bitmap@.is_bit_set(index as int));
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
                assert(bitmap@.is_bit_set(idx1 as int));
                assert(bitmap@.is_bit_set(idx2 as int));
                assert(bitmap@.usage() == 2);
            }
        }
    }
}

/// Test: alloc returns a previously-unset bit.
fn test_alloc_returns_unset_bit(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost pre_alloc = bitmap@;
        let a = bitmap.alloc();
        if let Ok(idx) = a {
            // The allocated bit was previously unset.
            assert(!pre_alloc.is_bit_set(idx as int));
            // Now it is set.
            assert(bitmap@.is_bit_set(idx as int));
        }
    }
}

//==================================================================================================
// Alloc Range Tests
//==================================================================================================

/// Test: alloc_range with size == number_of_bits on a partially filled bitmap should fail.
fn test_alloc_range_no_room() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        // Set one bit so the full-range alloc must fail.
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let ar = bitmap.alloc_range(8);
            assert(ar is Err);
        }
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
            assert(bitmap@.all_bits_set_in_range(start as int, (start + size) as int));
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
                    assert(bitmap@.is_bit_set(other as int));
                }
            }
        }
    }
}

/// Test: alloc_range on the full bitmap capacity should succeed on empty bitmap.
fn test_alloc_range_full_capacity(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(number_of_bits);
        if let Ok(start) = ar {
            assert(start == 0);
            assert(bitmap@.all_bits_set_in_range(0, number_of_bits as int));
            assert(bitmap@.usage() == number_of_bits as int);
        }
    }
}

/// Test: the range returned by alloc_range was previously free.
fn test_alloc_range_was_free(number_of_bits: usize, size: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        size > 0,
        size <= number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost pre = bitmap@;
        let ar = bitmap.alloc_range(size);
        if let Ok(start) = ar {
            assert(pre.all_bits_unset_in_range(start as int, (start + size) as int));
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

/// Test: number_of_bits (num_bits) remains constant across set/clear/alloc operations.
fn test_number_of_bits_constant(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost nb = bitmap@.num_bits;
        assert(nb == number_of_bits as int);

        let a = bitmap.alloc();
        if let Ok(_) = a {
            assert(bitmap@.num_bits == nb);
        }

        let s = bitmap.set(index);
        match s {
            Ok(()) => { assert(bitmap@.num_bits == nb); },
            Err(_) => { assert(bitmap@.num_bits == nb); },
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
            assert(bitmap@.is_bit_set(idx1 as int));

            let clr = bitmap.clear(idx1);
            if let Ok(()) = clr {
                assert(!bitmap@.is_bit_set(idx1 as int));
                assert(bitmap@.usage() == 0);

                // Re-allocate; should succeed since we freed a bit.
                let a2 = bitmap.alloc();
                if let Ok(idx2) = a2 {
                    assert(bitmap@.is_bit_set(idx2 as int));
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
                bitmap@.num_bits == 8,
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

/// Test: set/clear cycle restores usage to zero.
fn test_set_clear_restores_usage(number_of_bits: usize, index: usize)
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
                assert(!bitmap@.is_bit_set(index as int));
            }
        }
    }
}

/// Test: invariant is maintained after set.
fn test_inv_after_set(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(index);
        if let Ok(()) = s {
            assert(bitmap.inv());
        }
    }
}

/// Test: invariant is maintained after clear.
fn test_inv_after_clear(number_of_bits: usize, index: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        index < number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(index);
        if let Ok(()) = s {
            let c = bitmap.clear(index);
            if let Ok(()) = c {
                assert(bitmap.inv());
            }
        }
    }
}

/// Test: invariant is maintained after alloc.
fn test_inv_after_alloc(number_of_bits: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(_) = a {
            assert(bitmap.inv());
        }
    }
}

/// Test: invariant is maintained after alloc_range.
fn test_inv_after_alloc_range(number_of_bits: usize, size: usize)
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
            assert(bitmap.inv());
        }
    }
}

/// Test: three allocs usage equals 3.
fn test_three_allocs_usage()
{
    let result = Bitmap::new(32);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(_) = a1 {
            let a2 = bitmap.alloc();
            if let Ok(_) = a2 {
                let a3 = bitmap.alloc();
                if let Ok(idx3) = a3 {
                    assert(bitmap@.usage() == 3);

                    // Clear one bit.
                    let clr = bitmap.clear(idx3);
                    if let Ok(()) = clr {
                        assert(bitmap@.usage() == 2);
                    }
                }
            }
        }
    }
}

/// Test: alloc_range returns range where old bits were all unset.
fn test_alloc_range_old_unset(number_of_bits: usize, size: usize)
    requires
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        size > 0,
        size <= number_of_bits,
{
    let result = Bitmap::new(number_of_bits);
    if let Ok(mut bitmap) = result {
        let ghost pre = bitmap@;
        let ar = bitmap.alloc_range(size);
        if let Ok(start) = ar {
            // The range was free before allocation.
            assert(pre.all_bits_unset_in_range(start as int, (start + size) as int));
            // Now it's all set.
            assert(bitmap@.all_bits_set_in_range(start as int, (start + size) as int));
        }
    }
}

/// Test: when alloc fails, bitmap is unchanged (error frame).
fn test_alloc_error_frame()
{
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
            let ghost pre = bitmap@;
            // Bitmap is full; next alloc should fail and bitmap unchanged.
            let a = bitmap.alloc();
            match a {
                Ok(_) => {},
                Err(_) => {
                    assert(bitmap@ == pre);
                },
            }
        }
    }
}

//==================================================================================================
// Concrete-Input Correct Tests
//==================================================================================================

/// Concrete: set and clear bit 0 in a 64-bit bitmap.
fn test_set_clear_bit0() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let t0 = bitmap.test(0);
        if let Ok(v) = t0 { assert(!v); }

        let s = bitmap.set(0);
        if let Ok(()) = s {
            let t1 = bitmap.test(0);
            if let Ok(v) = t1 { assert(v); }

            let c = bitmap.clear(0);
            if let Ok(()) = c {
                let t2 = bitmap.test(0);
                if let Ok(v) = t2 { assert(!v); }
            }
        }
    }
}

/// Concrete: set and clear bit 7 (last bit of first byte).
fn test_set_clear_bit7() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(7);
        if let Ok(()) = s {
            assert(bitmap@.is_bit_set(7));
            assert(bitmap@.usage() == 1);

            let c = bitmap.clear(7);
            if let Ok(()) = c {
                assert(!bitmap@.is_bit_set(7));
                assert(bitmap@.usage() == 0);
            }
        }
    }
}

/// Concrete: set and clear bit 8 (first bit of second byte, cross-byte boundary).
fn test_set_clear_bit8() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(8);
        if let Ok(()) = s {
            assert(bitmap@.is_bit_set(8));
            assert(bitmap@.usage() == 1);

            let c = bitmap.clear(8);
            if let Ok(()) = c {
                assert(!bitmap@.is_bit_set(8));
                assert(bitmap@.usage() == 0);
            }
        }
    }
}

/// Concrete: setting bit 0 preserves bit 7 (same byte frame).
fn test_frame_same_byte() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            assert(!bitmap@.is_bit_set(7));
            let s2 = bitmap.set(7);
            if let Ok(()) = s2 {
                assert(bitmap@.is_bit_set(0));
                assert(bitmap@.is_bit_set(7));
            }
        }
    }
}

/// Concrete: setting bit 0 preserves bit 8 (cross-byte frame).
fn test_frame_cross_byte() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(0);
        if let Ok(()) = s1 {
            assert(!bitmap@.is_bit_set(8));
            let s2 = bitmap.set(8);
            if let Ok(()) = s2 {
                assert(bitmap@.is_bit_set(0));
                assert(bitmap@.is_bit_set(8));
            }
        }
    }
}

/// Concrete: alloc on an 8-bit bitmap.
fn test_alloc_concrete_8bit() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(idx) = a {
            assert(idx < 8);
            assert(bitmap@.is_bit_set(idx as int));
            assert(bitmap@.usage() == 1);
        }
    }
}

/// Concrete: alloc on a 64-bit bitmap.
fn test_alloc_concrete_64bit() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(idx) = a {
            assert(idx < 64);
            assert(bitmap@.is_bit_set(idx as int));
            assert(bitmap@.usage() == 1);
        }
    }
}

/// Concrete: alloc_range with size=1 on 64-bit bitmap.
fn test_alloc_range_size1() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(1);
        if let Ok(start) = ar {
            assert(start < 64);
            assert(bitmap@.is_bit_set(start as int));
            assert(bitmap@.usage() == 1);
        }
    }
}

/// Concrete: alloc_range with size=8 (one full byte) on 64-bit bitmap.
fn test_alloc_range_size8() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(8);
        if let Ok(start) = ar {
            assert(start + 8 <= 64);
            assert(bitmap@.all_bits_set_in_range(start as int, (start + 8) as int));
            assert(bitmap@.usage() == 8);
        }
    }
}

/// Concrete: alloc_range with size=3 on 64-bit bitmap.
fn test_alloc_range_size3() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(3);
        if let Ok(start) = ar {
            assert(start + 3 <= 64);
            assert(bitmap@.all_bits_set_in_range(start as int, (start + 3) as int));
            assert(bitmap@.usage() == 3);
        }
    }
}

/// Concrete: alloc_range full capacity on 8-bit bitmap.
fn test_alloc_range_full_8bit() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(8);
        if let Ok(start) = ar {
            assert(start == 0);
            assert(bitmap@.all_bits_set_in_range(0, 8));
            assert(bitmap@.usage() == 8);
        }
    }
}

/// Concrete: usage tracking with specific operations.
fn test_usage_concrete() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        assert(bitmap@.usage() == 0);

        let s = bitmap.set(5);
        if let Ok(()) = s {
            assert(bitmap@.usage() == 1);

            let s2 = bitmap.set(10);
            if let Ok(()) = s2 {
                assert(bitmap@.usage() == 2);

                let c = bitmap.clear(5);
                if let Ok(()) = c {
                    assert(bitmap@.usage() == 1);
                    assert(!bitmap@.is_bit_set(5));
                    assert(bitmap@.is_bit_set(10));
                }
            }
        }
    }
}

/// Concrete: index(0)→(0,0), index(7)→(0,7), index(8)→(1,0).
fn test_index_concrete() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let i0 = bitmap.index(0);
        if let Ok((w, b)) = i0 {
            assert(w == 0);
            assert(b == 0);
        }

        let i7 = bitmap.index(7);
        if let Ok((w, b)) = i7 {
            assert(w == 0);
            assert(b == 7);
        }

        let i8 = bitmap.index(8);
        if let Ok((w, b)) = i8 {
            assert(w == 1);
            assert(b == 0);
        }
    }
}

/// Concrete: index out of bounds on 8-bit bitmap.
fn test_index_oob_8bit() {
    let result = Bitmap::new(8);
    if let Ok(bitmap) = result {
        let i = bitmap.index(8);
        assert(i is Err);
    }
}

/// Concrete: num_bits stays constant through set/alloc/clear on 64-bit bitmap.
fn test_num_bits_constant_concrete() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        assert(bitmap@.num_bits == 64);

        let a = bitmap.alloc();
        if let Ok(idx) = a {
            assert(bitmap@.num_bits == 64);

            let c = bitmap.clear(idx);
            if let Ok(()) = c {
                assert(bitmap@.num_bits == 64);
            }
        }
    }
}

/// Concrete: set already-set bit 3 in 64-bit bitmap fails.
fn test_double_set_concrete() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s1 = bitmap.set(3);
        if let Ok(()) = s1 {
            let s2 = bitmap.set(3);
            assert(s2 is Err);
        }
    }
}

/// Concrete: clear already-cleared bit 3 in 64-bit bitmap fails.
fn test_double_clear_concrete() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let c = bitmap.clear(3);
        assert(c is Err);
    }
}

} // verus!
