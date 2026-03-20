// ERR13: On a full bitmap, claim alloc changes the bitmap state.
verus! {
fn test_err13_error_mutates() {
    let result = Bitmap::new(8);
    if let Ok(mut bitmap) = result {
        let mut i: usize = 0;
        while i < 8
            invariant
                0 <= i <= 8,
                bitmap.inv(),
                bitmap@.num_bits == 8,
                bitmap@.usage() == i as int,
            decreases 8 - i,
        {
            let a = bitmap.alloc();
            if let Ok(_) = a {
                i = i + 1;
            } else {
                break;
            }
        }

        if i == 8 {
            let ghost pre = bitmap@;
            let a = bitmap.alloc();
            match a {
                Ok(_) => {},
                Err(_) => {
                    // WRONG: on error, bitmap should be unchanged. Claiming usage changed.
                    assert(bitmap@.usage() != pre.usage());
                },
            }
        }
    }
}
} // verus!
