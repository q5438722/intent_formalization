// GAP06: After setting 7 bits in an 8-bit bitmap, alloc should still succeed.
verus! {
fn test_gap06_alloc_not_full_succeeds() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let mut i: usize = 0;
        while i < 7
            invariant
                0 <= i <= 7,
                bitmap.inv(),
                bitmap@.num_bits == 8,
                bitmap@.usage() == i as int,
            decreases 7 - i,
        {
            let a = bitmap.alloc();
            if let Ok(_) = a {
                i = i + 1;
            } else {
                break;
            }
        }
        if i == 7 {
            // CORRECT: usage is 7 < 8 = num_bits, so bitmap is not full.
            let a = bitmap.alloc();
            assert(a is Ok);
        }
    }
}
} // verus!
