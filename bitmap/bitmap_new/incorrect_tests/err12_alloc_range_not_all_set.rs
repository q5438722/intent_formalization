// ERR12: After alloc_range(4), claim the range is all unset (should be all set).
verus! {
fn test_err12_alloc_range_not_all_set() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(4);
        if let Ok(start) = ar {
            // WRONG: all bits in range should be set, not unset.
            assert(bitmap@.all_bits_unset_in_range(start as int, (start + 4) as int));
        }
    }
}
} // verus!
