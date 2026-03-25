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

// ===== Boundary Tests =====
// These tests violate the precondition (requires) by providing sequences
// where at least one element satisfies the predicate.

// SHOULD FAIL: One element in the sequence satisfies the predicate
proof fn test_boundary_one_element_satisfies() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x == 2;
    // Precondition violated: s[1] == 2, so pred(s[1]) is true
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
}

// SHOULD FAIL: All elements in the sequence satisfy the predicate
proof fn test_boundary_all_elements_satisfy() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    // Precondition violated: all elements are > 0
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
}

// SHOULD FAIL: Single-element sequence where the element satisfies the predicate
proof fn test_boundary_single_element_satisfies() {
    let s = seq![0int];
    let pred = |x: int| x == 0;
    // Precondition violated: the only element satisfies the predicate
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
}

// SHOULD FAIL: Last element satisfies the predicate (edge of drop_last recursion)
proof fn test_boundary_last_element_satisfies() {
    let s = seq![10int, 20, 30];
    let pred = |x: int| x == 30;
    // Precondition violated: s[2] == 30 satisfies pred
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
}

// SHOULD FAIL: First element satisfies the predicate
proof fn test_boundary_first_element_satisfies() {
    let s = seq![5int, 10, 15];
    let pred = |x: int| x == 5;
    // Precondition violated: s[0] == 5 satisfies pred
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
}

}
