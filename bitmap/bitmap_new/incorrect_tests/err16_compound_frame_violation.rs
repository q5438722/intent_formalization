// ERR16: set(0)+set(1)+clear(0) — wrongly assert bit 1 is unset.
verus! {
fn test_err16_compound_frame_violation() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s0 = bitmap.set(0);
        if let Ok(()) = s0 {
            let s1 = bitmap.set(1);
            if let Ok(()) = s1 {
                let c0 = bitmap.clear(0);
                if let Ok(()) = c0 {
                    // WRONG: clear(0) should not affect bit 1. Bit 1 should still be set.
                    assert(!bitmap@.is_bit_set(1));
                }
            }
        }
    }
}
} // verus!
