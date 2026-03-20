// ERR10: Two allocs on a 64-bit bitmap, claim they return the same index.
verus! {
fn test_err10_two_allocs_same() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(idx1) = a1 {
            let a2 = bitmap.alloc();
            if let Ok(idx2) = a2 {
                // WRONG: two allocs must return different indices.
                assert(idx1 == idx2);
            }
        }
    }
}
} // verus!
