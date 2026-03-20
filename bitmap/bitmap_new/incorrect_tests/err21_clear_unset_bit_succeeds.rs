// ERR21: Clear a bit that was never set — wrongly assert success.
verus! {
fn test_err21_clear_unset_bit_succeeds() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Bit 5 was never set. Clear should return Err.
        let c = bitmap.clear(5);
        // WRONG: clear on unset bit must fail.
        assert(c is Ok);
    }
}
} // verus!
