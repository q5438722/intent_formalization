use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemma (copied from source) =====

pub proof fn lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires forall |i: int| 0 <= i && i < s.len() ==> !pred(s[i])
    ensures  s.filter(pred) =~= Seq::<A>::empty()
    decreases s.len()
{
    reveal(Seq::filter);
    if s.len() != 0 {
        let subseq = s.drop_last();
        lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(subseq, pred);
        assert_seqs_equal!(s, subseq.push(s.last()));
    }
}

// ===== Logical Tests =====
// These tests assert properties NOT explicitly guaranteed by the specification:
// determinism, stronger inequalities, structural assumptions, cross-function misuse.

// SHOULD FAIL: The lemma says filter is empty, NOT that the original sequence is empty
proof fn test_logical_seq_must_be_empty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Unwarranted conclusion: original sequence must be empty
    assert(s.len() == 0);
}

// SHOULD FAIL: The lemma is about a specific predicate, not a different one
proof fn test_logical_different_predicate_filter_empty() {
    let s = seq![1int, 2, 3];
    let pred1 = |x: int| x > 10;
    let pred2 = |x: int| x > 0;
    // Valid for pred1: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred1);
    // Unwarranted conclusion: filter with pred2 is also empty
    assert(s.filter(pred2) =~= Seq::<int>::empty());
}

// SHOULD FAIL: The lemma is about sequence s, not about s extended with a new element
proof fn test_logical_extend_seq_still_empty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Unwarranted conclusion: appending 100 (which satisfies pred) still gives empty filter
    let s2 = s.push(100);
    assert(s2.filter(pred) =~= Seq::<int>::empty());
}

// SHOULD FAIL: The lemma does not establish that the sequence has any particular length
proof fn test_logical_seq_length_is_specific() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Unwarranted conclusion: sequence must have exactly 1 element
    assert(s.len() == 1);
}

// SHOULD FAIL: The lemma does not imply the predicate is universally false outside the sequence
proof fn test_logical_pred_false_outside_seq() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10 in s
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Unwarranted conclusion: pred is false for 100 (which is > 10)
    assert(!pred(100));
}

}
