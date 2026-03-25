use vstd::prelude::*;

fn main() {}

verus! {

// ===== Original specifications (copied for self-contained compilation) =====

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{ unimplemented!() }

pub proof fn true_pred_on_seq_implies_true_pred_on_filtered_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool, filter_pred: spec_fn(A) -> bool)
    requires forall |e: A| s.contains(e) ==> pred(e),
    ensures forall |e: A| s.filter(filter_pred).contains(e) ==> pred(e)
{
    assert(forall |e: A| s.filter(filter_pred).contains(e) ==> pred(e)) by {
        assert(forall |e: A| s.filter(filter_pred).contains(e) ==> #[trigger] s.contains(e)) by {
            seq_filter_is_a_subset_of_original_seq(s, filter_pred);
        }
        assert(forall |e: A| s.contains(e) ==> pred(e));
    }
}

// ===== Logical Tests =====

// --- Logical Test 1: Assert filter is identity (never removes elements) ---
// The spec guarantees filter(s) ⊆ s, but does NOT guarantee filter(s) == s.
// Asserting extensional equality with a selective predicate is a false claim.
// SHOULD FAIL
proof fn test_logical_filter_is_identity() {
    let s = seq![1int, 2int, 3int];
    let filter_pred = |x: int| x > 1;
    seq_filter_is_a_subset_of_original_seq::<int>(s, filter_pred);
    assert(s.filter(filter_pred) =~= s);
}

// --- Logical Test 2: Assert subset in BOTH directions (filter = original) ---
// The spec guarantees filter(s) ⊆ s, but NOT s ⊆ filter(s).
// Asserting the reverse inclusion would mean filtering never removes elements.
// SHOULD FAIL
proof fn test_logical_reverse_subset() {
    let s = seq![1int, 2int, 3int];
    let filter_pred = |x: int| x > 1;
    seq_filter_is_a_subset_of_original_seq::<int>(s, filter_pred);
    assert(forall |e: int| s.contains(e) ==> s.filter(filter_pred).contains(e));
}

// --- Logical Test 3: Cross-sequence reasoning ---
// The lemma establishes pred on filtered elements of a SPECIFIC sequence s1.
// Using it should NOT let us conclude pred on elements of an unrelated sequence s2
// whose elements do NOT satisfy pred.
// SHOULD FAIL
proof fn test_logical_cross_sequence() {
    let s1 = seq![1int, 2int, 3int];
    let s2 = seq![10int, 20int, 30int];
    let pred = |x: int| x < 5;      // s1 elements satisfy this; s2 elements do NOT
    let filter_pred = |x: int| true; // keeps all elements
    true_pred_on_seq_implies_true_pred_on_filtered_seq::<int>(s1, pred, filter_pred);
    // s2.filter(|x| true) = s2 = [10, 20, 30], and pred(10) = (10 < 5) = false
    assert(forall |e: int| s2.filter(filter_pred).contains(e) ==> pred(e));
}

}
