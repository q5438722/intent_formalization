// ERR23: After alloc, wrongly assert the returned bit is NOT set.
verus! {
fn test_err23_alloc_doesnt_set_bit() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(idx) = a {
            // WRONG: alloc guarantees the returned bit IS set.
            assert(!bitmap@.is_bit_set(idx as int));
        }
    }
}
} // verus!
