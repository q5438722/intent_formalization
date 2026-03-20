// ERR02: Incorrectly assert usage is 1 on a fresh bitmap.
verus! {
fn test_err02_new_usage_nonzero() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        // WRONG: usage should be 0.
        assert(bitmap@.usage() == 1);
    }
}
} // verus!
