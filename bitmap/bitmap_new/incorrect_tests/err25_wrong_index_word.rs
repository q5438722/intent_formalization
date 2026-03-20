// ERR25: index(9) should give word=1 (9/8=1), wrongly assert word=0.
verus! {
fn test_err25_wrong_index_word() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        let i = bitmap.index(9);
        if let Ok((word, bit)) = i {
            // WRONG: 9 / 8 = 1, not 0.
            assert(word == 0);
        }
    }
}
} // verus!
