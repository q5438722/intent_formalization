// ERR17: set(0)+set(1)+clear(0) — wrongly assert usage is 0 (should be 1).
verus! {
fn test_err17_wrong_usage_after_set_set_clear() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s0 = bitmap.set(0);
        if let Ok(()) = s0 {
            let s1 = bitmap.set(1);
            if let Ok(()) = s1 {
                let c0 = bitmap.clear(0);
                if let Ok(()) = c0 {
                    // WRONG: usage should be 1 (two sets, one clear), not 0.
                    assert(bitmap@.usage() == 0);
                }
            }
        }
    }
}
} // verus!
