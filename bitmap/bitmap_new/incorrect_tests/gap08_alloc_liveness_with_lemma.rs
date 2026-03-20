// GAP08: alloc on empty bitmap, using lemma to prove not full first.
verus! {
fn test_gap08_alloc_liveness_with_lemma() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        // Manually invoke the lemma: usage < num_bits implies not full.
        proof {
            bitmap@.lemma_usage_less_than_capacity_means_not_full();
        }
        let a = bitmap.alloc();
        assert(a is Ok);
    }
}
} // verus!
