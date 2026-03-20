// GAP07: set(0) then clear(0) must succeed (bit is set, so clear should work).
verus! {
fn test_gap07_clear_liveness() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // CORRECT: bit 0 is set, so clear(0) must succeed.
            let c = bitmap.clear(0);
            assert(c is Ok);
        }
    }
}
} // verus!
