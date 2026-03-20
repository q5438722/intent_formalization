// GAP03: alloc_range(8) on 8-bit bitmap makes it full.
// Tests if spec can derive is_full() from all_bits_set_in_range(0, 8) + num_bits==8.
verus! {
fn test_gap03_alloc_range_full_is_full() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(8);
        if let Ok(start) = ar {
            // CORRECT: if start==0 and all bits in [0,8) are set and num_bits==8, bitmap is full.
            assert(bitmap@.is_full());
        }
    }
}
} // verus!
