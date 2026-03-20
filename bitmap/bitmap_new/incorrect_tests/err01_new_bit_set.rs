// ERR01: Incorrectly assert a newly created bitmap has a bit set.
verus! {
fn test_err01_new_bit_set() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        // WRONG: new bitmap should be empty, this should fail.
        assert(bitmap@.is_bit_set(0));
    }
}
} // verus!
