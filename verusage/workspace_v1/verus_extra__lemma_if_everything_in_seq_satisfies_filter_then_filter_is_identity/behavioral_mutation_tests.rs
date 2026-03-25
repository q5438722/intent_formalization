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
// BEHAVIORAL MUTATION TESTS: Valid inputs, wrong output claims
// ============================================================

// Test 1: Assert filter result is NOT equal to original (negates ensures)
// SHOULD FAIL - the lemma guarantees filter == s
proof fn test_mutation_filter_not_equal_to_original() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred) != s);
}

// Test 2: Assert filter result is empty
// SHOULD FAIL - filter should equal s which has length 3
proof fn test_mutation_filter_gives_empty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred).len() == 0);
}

// Test 3: Assert filter result has shorter length
// SHOULD FAIL - filter should preserve length when all elements satisfy pred
proof fn test_mutation_filter_shorter_length() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred).len() < s.len());
}

// Test 4: Assert filter result equals a different sequence
// SHOULD FAIL - filter should equal s = [1,2,3], not [3,2,1]
proof fn test_mutation_filter_equals_wrong_seq() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    let wrong = seq![3int, 2, 1];
    assert(s.filter(pred) =~= wrong);
}

// Test 5: Assert filter changes an element value
// SHOULD FAIL - filter preserves all elements, so first element is still 1
proof fn test_mutation_filter_changes_element() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred)[0] == 99);
}

}
