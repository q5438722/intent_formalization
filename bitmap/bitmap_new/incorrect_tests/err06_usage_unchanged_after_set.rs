// ERR06: After setting bit 0 in a 64-bit bitmap, claim usage is still 0.
verus! {
fn test_err06_usage_unchanged_after_set() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: usage should be 1 after set, not 0.
            assert(bitmap@.usage() == 0);
        }
    }
}
} // verus!
