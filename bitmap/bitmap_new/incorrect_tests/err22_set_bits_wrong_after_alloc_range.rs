// ERR22: alloc_range(2), wrongly assert set_bits equals a single-element set.
verus! {
fn test_err22_set_bits_wrong_after_alloc_range() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(2);
        if let Ok(start) = ar {
            // WRONG: usage should be 2, not 1. The range sets TWO bits.
            assert(bitmap@.usage() == 1);
        }
    }
}
} // verus!
