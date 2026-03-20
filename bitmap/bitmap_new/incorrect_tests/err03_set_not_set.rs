// ERR03: After setting bit 5 in a 64-bit bitmap, incorrectly assert it is NOT set.
verus! {
fn test_err03_set_not_set() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(5);
        if let Ok(()) = s {
            // WRONG: bit 5 was just set, should be true not false.
            assert(!bitmap@.is_bit_set(5));
        }
    }
}
} // verus!
