// ERR04: After clearing bit 5 in a 64-bit bitmap, incorrectly assert it is still set.
verus! {
fn test_err04_clear_not_cleared() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(5);
        if let Ok(()) = s {
            let c = bitmap.clear(5);
            if let Ok(()) = c {
                // WRONG: bit 5 was just cleared, should be false.
                assert(bitmap@.is_bit_set(5));
            }
        }
    }
}
} // verus!
