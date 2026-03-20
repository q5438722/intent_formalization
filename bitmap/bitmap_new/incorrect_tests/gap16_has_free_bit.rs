// GAP16: After set(0) on 64-bit bitmap, prove has_free_bit (63 bits still free).
verus! {
fn test_gap16_has_free_bit() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let s = bitmap.set(0);
        if let Ok(()) = s {
            assert(bitmap@.has_free_bit());
        }
    }
}
} // verus!
