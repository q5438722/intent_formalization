use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{unimplemented!()}

pub proof fn seq_filter_preserves_no_duplicates<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires s.no_duplicates(),
    ensures s.filter(pred).no_duplicates()
    decreases s.len()
{
    reveal(Seq::filter);
    if s.len() != 0 {
        seq_filter_preserves_no_duplicates(s.drop_last(), pred);
        if pred(s.last()) {
            seq_filter_is_a_subset_of_original_seq(s.drop_last(), pred);
        }
    }
}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutated output — claim filter preserves length.
// Filtering with a predicate that rejects some elements SHOULD reduce length.
// The spec only guarantees no_duplicates, NOT length preservation.
// SHOULD FAIL
proof fn test_mutation_filter_preserves_length()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 1;
    seq_filter_preserves_no_duplicates(s, pred);
    assert(s.filter(pred).len() == s.len());
}

// Test 2: Mutated relation — claim every element of original is in the filter (reverse subset).
// The axiom only guarantees filter ⊆ original, NOT original ⊆ filter.
// Element 1 does not satisfy x > 1, so it should NOT be in the filter.
// SHOULD FAIL
proof fn test_mutation_reverse_subset()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 1;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).contains(1int));
}

// Test 3: Mutated output — claim filter result HAS duplicates.
// The ensures clause guarantees no_duplicates on filter result.
// Asserting the negation directly contradicts the postcondition.
// SHOULD FAIL
proof fn test_mutation_filter_has_duplicates()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    seq_filter_preserves_no_duplicates(s, pred);
    assert(!s.filter(pred).no_duplicates());
}

}
