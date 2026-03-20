// GAP04: alloc on empty bitmap must succeed (not full => must find a bit).
// Tests if spec's Err condition (is_full) is strong enough to guarantee success.
verus! {
fn test_gap04_alloc_liveness_empty() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // CORRECT: empty bitmap is not full, so alloc must succeed.
        let a = bitmap.alloc();
        assert(a is Ok);
    }
}
} // verus!
