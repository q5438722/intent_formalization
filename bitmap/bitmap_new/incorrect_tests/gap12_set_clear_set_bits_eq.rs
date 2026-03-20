// GAP12: After set+clear, prove set_bits equals empty set using extensional equality.
verus! {
fn test_gap12_set_clear_set_bits_eq() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            let c = bitmap.clear(0);
            if let Ok(()) = c {
                // Try to prove set_bits equals empty through extensional equality.
                assert(bitmap@.set_bits =~= Set::<int>::empty());
            }
        }
    }
}
} // verus!
