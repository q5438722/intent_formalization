// ERR24: alloc_range(4) on 64-bit, wrongly assert start + size > num_bits.
verus! {
fn test_err24_alloc_range_wrong_start_bound() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(4);
        if let Ok(start) = ar {
            // WRONG: spec guarantees start + size <= num_bits.
            assert(start + 4 > 64);
        }
    }
}
} // verus!
