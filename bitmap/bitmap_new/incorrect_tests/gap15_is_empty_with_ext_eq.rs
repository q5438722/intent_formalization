// GAP15: After set(0)+clear(0), prove is_empty() by first asserting extensional equality.
verus! {
fn test_gap15_is_empty_with_ext_eq() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // Trigger extensional equality first...
                assert(bitmap@.set_bits =~= Set::<int>::empty());
                // ...then is_empty() should follow.
                assert(bitmap@.is_empty());
            }
        }
    }
}
} // verus!
