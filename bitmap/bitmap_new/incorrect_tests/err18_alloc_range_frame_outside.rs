// ERR18: alloc_range(4) on empty bitmap, wrongly assert a bit OUTSIDE the range is set.
verus! {
fn test_err18_alloc_range_frame_outside() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(4);
        if let Ok(start) = ar {
            if start + 4 < 64 {
                // WRONG: bit start+4 is outside the allocated range and should be unset.
                assert(bitmap@.is_bit_set((start + 4) as int));
            }
        }
    }
}
} // verus!
