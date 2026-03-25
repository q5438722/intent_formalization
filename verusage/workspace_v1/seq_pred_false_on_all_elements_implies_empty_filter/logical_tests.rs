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

// === LOGICAL TEST 1 ===
// Try to prove that when pred is false on all elements of s,
// the sequence itself must be empty (s.len() == 0).
// This is FALSE: a non-empty sequence can have pred false on all its elements.
// The spec says nothing about s.len(); only about filter length.
// SHOULD FAIL
proof fn logical_test_1_sequence_must_be_empty() {
    let s: Seq<int> = seq![1, 2, 3];
    let pred = |x: int| x > 100;
    // Precondition holds: pred is false on all elements
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Incorrectly conclude that s itself must be empty
    assert(s.len() == 0);
}

// === LOGICAL TEST 2 ===
// Try to prove universal falsity of pred: if pred is false on all elements
// of s, then pred must be false EVERYWHERE (on all int values).
// This is FALSE: pred can be true on values not in s.
// SHOULD FAIL
proof fn logical_test_2_pred_false_universally() {
    let s: Seq<int> = seq![1, 2, 3];
    let pred = |x: int| x > 100;
    // Precondition holds: pred is false on 1, 2, 3
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Incorrectly conclude that pred is false on ALL integers (e.g., 200)
    assert(!pred(200));
}

// === LOGICAL TEST 3 ===
// Try cross-sequence reasoning: use the function on one sequence s1,
// then conclude that a DIFFERENT sequence s2 also has an empty filter.
// The postcondition only applies to the sequence passed as argument.
// SHOULD FAIL
proof fn logical_test_3_cross_sequence_reasoning() {
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![200, 300];
    let pred = |x: int| x > 100;
    // Precondition holds for s1: pred is false on 1, 2, 3
    seq_pred_false_on_all_elements_implies_empty_filter(s1, pred);
    // Incorrectly conclude that s2.filter(pred) is also empty
    // (s2 contains 200, 300 which satisfy pred)
    assert(s2.filter(pred).len() == 0);
}

}
