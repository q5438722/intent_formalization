use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{unimplemented!()}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutated relation — reverse the subset direction.
// The spec guarantees filter(pred).contains(e) ==> s.contains(e).
// Here we assert the CONVERSE: s.contains(e) ==> filter(pred).contains(e).
// Element 1 does not satisfy x > 1, so it is in s but NOT in filter.
// SHOULD FAIL
proof fn test_mutation_reverse_containment()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 1;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).contains(1int));
}

// Test 2: Mutated output — claim filter preserves length.
// Filtering with a selective predicate removes elements, so the length should decrease.
// The spec says nothing about length equality; asserting it should fail.
// SHOULD FAIL
proof fn test_mutation_length_preservation()
{
    let s: Seq<int> = seq![1int, 2, 3, 4, 5];
    let pred = |x: int| x > 3;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).len() == s.len());
}

// Test 3: Mutated output — claim filter equals original sequence.
// When the predicate rejects some elements, the filter is a proper subset,
// so the filtered sequence should NOT be extensionally equal to the original.
// SHOULD FAIL
proof fn test_mutation_filter_equals_original()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x != 2;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred) =~= s);
}

}
