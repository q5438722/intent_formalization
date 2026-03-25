use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{unimplemented!()}

// ========== LOGICAL TESTS ==========

// Test 1: Soundness — try to derive `false` from the spec.
// If the external_body axiom introduces an inconsistency, calling it
// would allow deriving `false` (and therefore anything).
// This verifies the axiom's ensures clause is not self-contradictory.
// SHOULD FAIL
proof fn test_logical_derive_false()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(false);
}

// Test 2: Stronger inequality — filter length exceeds original.
// The spec only guarantees that filtered elements are in the original.
// It does NOT explicitly bound the filter's length relative to s.len().
// Asserting filter.len() > s.len() tests whether this stronger property leaks.
// SHOULD FAIL
proof fn test_logical_filter_length_exceeds_original()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    assert(s.filter(pred).len() > s.len());
}

// Test 3: Cross-function misuse — assert index correspondence between
// original and filtered sequence that is NOT guaranteed.
// The spec guarantees filtered elements exist in the original, but does NOT
// guarantee that filter[0] == s[0] when the first element is filtered out.
// SHOULD FAIL
proof fn test_logical_index_correspondence()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 1;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    // The first element of filter should be 2 (first matching), not 1 (first of s).
    // Asserting filter[0] == s[0] (which is 1) should fail.
    assert(s.filter(pred).len() > 0);
    assert(s.filter(pred)[0] == s[0]);
}

}
