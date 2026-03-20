// ERR20: Two alloc_range(2), wrongly assert their ranges overlap.
verus! {
fn test_err20_two_alloc_ranges_overlap() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let ar1 = bitmap.alloc_range(2);
        if let Ok(s1) = ar1 {
            let ar2 = bitmap.alloc_range(2);
            if let Ok(s2) = ar2 {
                // WRONG: the second range must start where old bits were unset.
                // s1 range is [s1, s1+2), s2 range must not overlap (old bits in s1 range are set).
                // So s2 can't equal s1.
                assert(s1 == s2);
            }
        }
    }
}
} // verus!
