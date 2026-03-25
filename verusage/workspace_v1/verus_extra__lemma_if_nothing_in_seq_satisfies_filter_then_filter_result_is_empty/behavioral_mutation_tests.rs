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

// ===== Behavioral Mutation Tests =====
// These tests start from valid inputs (preconditions satisfied) but assert
// mutated/incorrect conclusions about the output.

// SHOULD FAIL: Assert the filter result is non-empty (negated ensures)
proof fn test_mutation_filter_nonempty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Mutated assertion: claim filter has elements
    assert(s.filter(pred).len() > 0);
}

// SHOULD FAIL: Assert the filter result equals the original sequence
proof fn test_mutation_filter_equals_original() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Mutated assertion: claim filter equals original (should be empty, not original)
    assert(s.filter(pred) =~= s);
}

// SHOULD FAIL: Assert the filter result has exactly one element
proof fn test_mutation_filter_has_one_element() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Mutated assertion: claim filter result has length 1
    assert(s.filter(pred).len() == 1);
}

// SHOULD FAIL: Assert the filter result equals a non-empty specific sequence
proof fn test_mutation_filter_equals_wrong_seq() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Mutated assertion: claim filter equals seq![1]
    assert(s.filter(pred) =~= seq![1int]);
}

// SHOULD FAIL: Assert the filter result length equals the original sequence length
proof fn test_mutation_filter_len_equals_original_len() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    // Valid: no element > 10
    lemma_if_nothing_in_seq_satisfies_filter_then_filter_result_is_empty(s, pred);
    // Mutated assertion: claim filter length equals original length
    assert(s.filter(pred).len() == s.len());
}

}
