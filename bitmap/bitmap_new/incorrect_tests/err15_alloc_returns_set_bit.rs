// ERR15: Set bit 0, then alloc. Wrongly assert alloc returned index 0.
verus! {
fn test_err15_alloc_returns_set_bit() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let a = bitmap.alloc();
            if let Ok(idx) = a {
                // WRONG: alloc must return a previously-UNSET bit, but bit 0 is set.
                assert(idx == 0);
            }
        }
    }
}
} // verus!
