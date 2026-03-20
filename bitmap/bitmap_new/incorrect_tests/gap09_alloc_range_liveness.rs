// GAP09: alloc_range(1) on empty 64-bit bitmap should succeed.
verus! {
fn test_gap09_alloc_range_liveness() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(1);
        assert(ar is Ok);
    }
}
} // verus!
