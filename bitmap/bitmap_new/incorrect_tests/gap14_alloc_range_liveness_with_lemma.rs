// GAP14: alloc_range(1) on empty bitmap with lemma help.
verus! {
fn test_gap14_alloc_range_liveness_with_lemma() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
            bitmap@.lemma_has_free_bit_implies_exists_free_range_1();
        }
        let ar = bitmap.alloc_range(1);
        assert(ar is Ok);
    }
}
} // verus!
