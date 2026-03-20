// GAP10: An empty bitmap with num_bits > 0 should not be full.
verus! {
fn test_gap10_empty_not_full() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        assert(!bitmap@.is_full());
    }
}
} // verus!
