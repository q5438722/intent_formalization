// ERR08: After alloc_range(4) on a 64-bit bitmap, claim usage is 5.
verus! {
fn test_err08_alloc_range_wrong_usage() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar = bitmap.alloc_range(4);
        if let Ok(_) = ar {
            // WRONG: usage should be exactly 4, not 5.
            assert(bitmap@.usage() == 5);
        }
    }
}
} // verus!
