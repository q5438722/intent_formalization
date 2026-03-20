// GAP13: Prove !is_full() on empty bitmap using lemma.
verus! {
fn test_gap13_empty_not_full_with_lemma() {
    let result = Bitmap::new(64);
    if let Ok(bitmap) = result {
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
        }
        assert(!bitmap@.is_full());
    }
}
} // verus!
