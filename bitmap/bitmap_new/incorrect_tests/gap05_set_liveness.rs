// GAP05: set(0) on empty 64-bit bitmap must succeed.
// Tests if spec's Err condition is strong enough to guarantee success.
verus! {
fn test_gap05_set_liveness() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // CORRECT: index 0 < 64 and bit 0 is unset, so set(0) must succeed.
        let s = bitmap.set(0);
        assert(s is Ok);
    }
}
} // verus!
