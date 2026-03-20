// ERR14: Off-by-one - alloc_range(3) sets bits [start,start+3), wrongly assert start+3 is set.
verus! {
fn test_err14_alloc_range_off_by_one() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(3);
        if let Ok(start) = ar {
            if start + 3 < 64 {
                // WRONG: bit start+3 is OUTSIDE the range [start, start+3).
                assert(bitmap@.is_bit_set((start + 3) as int));
            }
        }
    }
}
} // verus!
