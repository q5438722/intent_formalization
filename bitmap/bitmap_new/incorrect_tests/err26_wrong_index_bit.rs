// ERR26: index(9) should give bit=1 (9%8=1), wrongly assert bit=2.
verus! {
fn test_err26_wrong_index_bit() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let i = bitmap.index(9);
        if let Ok((word, bit)) = i {
            // WRONG: 9 % 8 = 1, not 2.
            assert(bit == 2);
        }
    }
}
} // verus!
