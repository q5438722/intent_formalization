// GAP11: After setting a bit, bitmap should not be empty.
verus! {
fn test_gap11_set_not_empty() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            assert(!bitmap@.is_empty());
        }
    }
}
} // verus!
