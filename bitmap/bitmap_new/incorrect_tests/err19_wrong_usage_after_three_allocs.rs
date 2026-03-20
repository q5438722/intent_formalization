// ERR19: Three allocs, wrongly assert usage is 2 (should be 3).
verus! {
fn test_err19_wrong_usage_after_three_allocs() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a1 = bitmap.alloc();
        if let Ok(_) = a1 {
            let a2 = bitmap.alloc();
            if let Ok(_) = a2 {
                let a3 = bitmap.alloc();
                if let Ok(_) = a3 {
                    // WRONG: usage should be 3 after three allocs, not 2.
                    assert(bitmap@.usage() == 2);
                }
            }
        }
    }
}
} // verus!
