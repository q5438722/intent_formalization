// ERR07: After alloc on a 64-bit bitmap, claim the returned index >= 64.
verus! {
fn test_err07_alloc_out_of_range() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(idx) = a {
            // WRONG: alloc should return idx < 64.
            assert(idx >= 64);
        }
    }
}
} // verus!
