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

// ===== Logical Tests =====
// These tests assert properties NOT explicitly guaranteed by the specification:
// stronger claims, structural assumptions, cross-predicate reasoning.

// SHOULD FAIL: The lemma does not imply skip(i) equals the original sequence
proof fn test_logical_skip_equals_original() {
    let s = seq![1int, 2, 3, 20];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Unwarranted: skip(3) should be seq![20], not seq![1,2,3,20]
    assert(s.skip(3) =~= s);
}

// SHOULD FAIL: The lemma does not guarantee the filter result is empty
proof fn test_logical_filter_must_be_empty() {
    let s = seq![1int, 2, 3, 20];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Unwarranted: filter should contain 20, not be empty
    assert(s.filter(pred).len() == 0);
}

// SHOULD FAIL: The lemma about pred1 does not transfer to a different predicate pred2
proof fn test_logical_different_predicate() {
    let s = seq![1int, 2, 3, 20];
    let pred1 = |x: int| x > 10;
    let pred2 = |x: int| x > 0;
    lemma_filter_skip_rejected(s, pred1, 3);
    // Unwarranted: conclusion about pred1 does not apply to pred2
    assert(s.filter(pred2) == s.skip(3).filter(pred2));
}

// SHOULD FAIL: The lemma does not guarantee properties about extended sequences
proof fn test_logical_extend_sequence() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Unwarranted: appending 100 changes the filter result
    let s2 = s.push(100int);
    assert(s2.filter(pred) == s2.skip(3).filter(pred));
}

// SHOULD FAIL: The lemma does not imply the original sequence length is 0
proof fn test_logical_seq_is_empty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 3);
    // Unwarranted: the sequence clearly has 3 elements
    assert(s.len() == 0);
}

}
