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

// ===== Behavioral Mutation Tests =====

// --- Mutation Test 1: Negate the postcondition ---
// Assert there EXISTS an element in the filtered seq that does NOT satisfy pred.
// This directly contradicts the ensures clause.
// SHOULD FAIL
proof fn test_mutation_negate_postcondition() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    let filter_pred = |x: int| x > 1;
    true_pred_on_seq_implies_true_pred_on_filtered_seq::<int>(s, pred, filter_pred);
    assert(exists |e: int| s.filter(filter_pred).contains(e) && !pred(e));
}

// --- Mutation Test 2: Reverse the postcondition direction ---
// Assert that pred(e) implies e is contained in the filtered sequence.
// This is the converse of the actual postcondition and is NOT guaranteed.
// SHOULD FAIL
proof fn test_mutation_reverse_postcondition() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    let filter_pred = |x: int| x > 1;
    true_pred_on_seq_implies_true_pred_on_filtered_seq::<int>(s, pred, filter_pred);
    // converse: pred(e) ==> contained in filter
    assert(forall |e: int| pred(e) ==> s.filter(filter_pred).contains(e));
}

// --- Mutation Test 3: Assert filter preserves sequence length ---
// Filtering may remove elements, so length should generally decrease.
// Asserting equality of lengths is a mutated (incorrect) behavioral claim.
// SHOULD FAIL
proof fn test_mutation_length_preserved() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    let filter_pred = |x: int| x > 1;
    true_pred_on_seq_implies_true_pred_on_filtered_seq::<int>(s, pred, filter_pred);
    assert(s.filter(filter_pred).len() == s.len());
}

}
