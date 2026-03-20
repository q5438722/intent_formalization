// ERR11: Set bit 3, clear bit 3, claim usage is 2 (should be 0).
verus! {
fn test_err11_clear_increases_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(3);
        if let Ok(()) = s {
            assert(bitmap@.usage() == 1);
            let c = bitmap.clear(3);
            if let Ok(()) = c {
                // WRONG: usage should decrease to 0, not increase to 2.
                assert(bitmap@.usage() == 2);
            }
        }
    }
}
} // verus!
