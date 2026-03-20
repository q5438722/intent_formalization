// GAP02: After set(0)+clear(0) on fresh bitmap, bitmap should be empty again.
// Tests if spec can prove Set::empty().insert(0).remove(0) == Set::empty().
verus! {
fn test_gap02_set_clear_is_empty() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // CORRECT: set_bits should be {}.insert(0).remove(0) == {}.
                assert(bitmap@.is_empty());
            }
        }
    }
}
} // verus!
