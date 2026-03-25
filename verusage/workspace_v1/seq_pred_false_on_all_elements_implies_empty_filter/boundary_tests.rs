use vstd::prelude::*;

fn main() {}

verus! {

// Original function under test
proof fn seq_pred_false_on_all_elements_implies_empty_filter<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires forall |e: A| #![auto] s.contains(e) ==> !pred(e),
    ensures s.filter(pred).len() == 0,
    decreases s.len()
{
    reveal(Seq::filter);
    if s.len() != 0 {
        let subseq = s.drop_last();
        assert(forall |e: A| subseq.contains(e) ==> !pred(e)) by {
            assert(forall |i: int| 0 <= i < subseq.len() ==> s.contains(#[trigger] s[i]) ==> !pred(subseq[i]));
        }
        seq_pred_false_on_all_elements_implies_empty_filter(subseq, pred);
        assert(subseq.filter(pred) == s.filter(pred)) by {
            assert(!pred(s.last())) by {
                assert(s.contains(s.last()) ==> !pred(s.last()));
            };
        }
    }
}

// === BOUNDARY TEST 1 ===
// Call function with a predicate that is TRUE on an element in the sequence.
// The precondition requires pred to be false on ALL contained elements.
// This violates the precondition; the call should be rejected.
// SHOULD FAIL
proof fn boundary_test_1_pred_true_on_element() {
    let s: Seq<int> = seq![1, 2, 3];
    let pred = |x: int| x == 2;
    // pred(2) == true, but 2 is in s → violates requires
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
}

// === BOUNDARY TEST 2 ===
// Call function with an always-true predicate on a non-empty sequence.
// Every element satisfies pred, directly violating the precondition.
// SHOULD FAIL
proof fn boundary_test_2_always_true_pred() {
    let s: Seq<int> = seq![42];
    let pred = |x: int| true;
    // pred is true for all elements → violates requires
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
}

// === BOUNDARY TEST 3 ===
// Try to assert that filter is empty on a sequence WITHOUT establishing the
// precondition (no call to the function, just a direct assertion).
// Without the precondition, filter could be non-empty.
// SHOULD FAIL
proof fn boundary_test_3_assert_filter_empty_without_precondition() {
    let s: Seq<int> = seq![1, 2, 3];
    let pred = |x: int| x > 0;
    // We do NOT call the function and do NOT establish the precondition.
    // Directly asserting the filter is empty should fail.
    assert(s.filter(pred).len() == 0);
}

}
