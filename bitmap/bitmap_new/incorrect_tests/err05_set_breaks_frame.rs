// ERR05: After setting bit 0 in a 64-bit bitmap, incorrectly assert bit 1 is also set.
verus! {
fn test_err05_set_breaks_frame() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            // WRONG: only bit 0 was set, bit 1 should still be unset.
            assert(bitmap@.is_bit_set(1));
        }
    }
}
} // verus!
