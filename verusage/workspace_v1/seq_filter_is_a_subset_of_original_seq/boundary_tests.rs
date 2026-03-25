use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{unimplemented!()}

// ========== BOUNDARY TESTS ==========

// Test 1: Empty sequence — assert filter produces a non-empty result.
// An empty sequence filtered by any predicate should remain empty.
// The ensures are vacuously true for empty sequences; this checks
// whether the spec allows deriving a non-empty filter from nothing.
// SHOULD FAIL
proof fn test_boundary_empty_seq_nonempty_filter()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| true;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).len() > 0);
}

// Test 2: Single element not matching predicate — assert it is still in filter.
// For s = seq![5] with pred = |x| x > 10, element 5 does not satisfy pred.
// The spec guarantees filter ⊆ original, but does NOT guarantee the converse.
// Asserting the non-matching element is in the filter should be rejected.
// SHOULD FAIL
proof fn test_boundary_single_element_not_matching()
{
    let s: Seq<int> = seq![5int];
    let pred = |x: int| x > 10;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).contains(5int));
}

// Test 3: All elements filtered out — assert filter still has positive length.
// When no elements satisfy the predicate, the filter should be empty.
// This tests whether the spec permits a non-empty filter when pred rejects all.
// SHOULD FAIL
proof fn test_boundary_all_filtered_out_positive_length()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 100;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).len() > 0);
}

}
