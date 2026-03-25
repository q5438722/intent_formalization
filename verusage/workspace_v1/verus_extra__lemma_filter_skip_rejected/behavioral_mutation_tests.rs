use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemma (copied from source) =====

pub proof fn lemma_filter_skip_rejected<A>(s: Seq<A>, pred: spec_fn(A) -> bool, i: int)
    requires
        0 <= i <= s.len(),
        forall |j| 0 <= j < i ==> !pred(s[j]),
    ensures
        s.filter(pred) == s.skip(i).filter(pred)
    decreases
        s.len()
{
    reveal(Seq::filter);
    if s.len() == 0 {
        assert(s.skip(i) =~= s);
    }
    else if i < s.len() {
        assert(s.skip(i).drop_last() =~= s.drop_last().skip(i));
        lemma_filter_skip_rejected(s.drop_last(), pred, i);
    }
    else {
        assert(s.skip(i) =~= s.drop_last().skip(i - 1));
        lemma_filter_skip_rejected(s.drop_last(), pred, i - 1);
    }
}

// ===== Behavioral Mutation Tests =====
// These tests use valid inputs but assert mutated/incorrect postconditions.

// SHOULD FAIL: Negate the postcondition — assert filter results are NOT equal
proof fn test_mutation_negate_postcondition() {
    let s = seq![1int, 2, 3, 20, 30];
    let pred = |x: int| x > 10;
    // Valid: first 3 elements (1,2,3) don't satisfy x > 10, i = 3
    lemma_filter_skip_rejected(s, pred, 3);
    // Mutated: claim the filter results differ
    assert(s.filter(pred) !== s.skip(3).filter(pred));
}

// SHOULD FAIL: Assert filter equals original sequence (wrong output)
proof fn test_mutation_filter_equals_original() {
    let s = seq![1int, 2, 3, 20, 30];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Mutated: filter should be seq![20, 30], not seq![1, 2, 3, 20, 30]
    assert(s.filter(pred) =~= s);
}

// SHOULD FAIL: Assert filter result is empty when elements after i satisfy pred
proof fn test_mutation_filter_result_is_empty() {
    let s = seq![1int, 2, 3, 20, 30];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Mutated: filter should be seq![20, 30], not empty
    assert(s.filter(pred) =~= Seq::<int>::empty());
}

// SHOULD FAIL: Assert skip(i+1) gives same filter when element at i satisfies pred
proof fn test_mutation_skip_one_more() {
    let s = seq![1int, 2, 3, 20, 30];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // s[3] = 20 satisfies pred, so skip(4).filter != s.filter
    assert(s.filter(pred) == s.skip(4).filter(pred));
}

// SHOULD FAIL: Assert filter length equals original length
proof fn test_mutation_filter_len_equals_original() {
    let s = seq![1int, 2, 3, 20, 30];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Filter keeps only 20, 30 (len 2), not original len 5
    assert(s.filter(pred).len() == s.len());
}

}
