use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemma under test
pub proof fn lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires forall |i: int| 0 <= i && i < s.len() ==> pred(s[i])
    ensures  s.filter(pred) == s
    decreases s.len()
{
    reveal(Seq::filter);
    if s.len() != 0 {
        let subseq = s.drop_last();
        lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(subseq, pred);
        assert_seqs_equal!(s, subseq.push(s.last()));
    }
}

// ============================================================
// BOUNDARY TESTS: Violate preconditions (requires)
// ============================================================

// Test 1: Some elements do NOT satisfy the predicate
// SHOULD FAIL - precondition violated (elements 1, 2 don't satisfy x > 2)
proof fn test_boundary_not_all_elements_satisfy_pred() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 2;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// Test 2: No elements satisfy the predicate
// SHOULD FAIL - precondition violated (no element satisfies x > 10)
proof fn test_boundary_no_elements_satisfy_pred() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// Test 3: Single element that does NOT satisfy the predicate
// SHOULD FAIL - precondition violated (0 does not satisfy x > 0)
proof fn test_boundary_single_element_fails_pred() {
    let s = seq![0int];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// Test 4: Only the last element fails the predicate
// SHOULD FAIL - precondition violated (last element -1 doesn't satisfy x >= 0)
proof fn test_boundary_last_element_fails() {
    let s = seq![1int, 2, 3, -1];
    let pred = |x: int| x >= 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// Test 5: Only the first element fails the predicate
// SHOULD FAIL - precondition violated (first element -1 doesn't satisfy x >= 0)
proof fn test_boundary_first_element_fails() {
    let s = seq![-1int, 1, 2, 3];
    let pred = |x: int| x >= 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

}
