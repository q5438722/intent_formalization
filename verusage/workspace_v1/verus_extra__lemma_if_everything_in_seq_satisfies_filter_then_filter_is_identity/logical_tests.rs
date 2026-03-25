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
// LOGICAL TESTS: Properties not entailed by the specification
// ============================================================

// Test 1: Try to prove filter is identity WITHOUT calling the lemma
// SHOULD FAIL - the conclusion requires the lemma's proof
proof fn test_logical_conclusion_without_lemma() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    // Do NOT call the lemma - just assert the conclusion directly
    assert(s.filter(pred) == s);
}

// Test 2: Try to derive that pred holds universally (not just for seq elements)
// SHOULD FAIL - the lemma only proves pred holds for elements in s, not for all ints
proof fn test_logical_pred_universally_true() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    // Try to claim pred holds for a value NOT in the sequence
    assert(pred(-5));
}

// Test 3: Cross-sequence misuse - apply conclusion to a different sequence
// SHOULD FAIL - the lemma only applies to sequence s, not s2
proof fn test_logical_cross_sequence_misuse() {
    let s1 = seq![1int, 2, 3];
    let s2 = seq![1int, -1, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s1, pred);
    // s2 has element -1 that doesn't satisfy pred, so filter(s2, pred) != s2
    assert(s2.filter(pred) == s2);
}

// Test 4: Try to use the lemma with a DIFFERENT predicate than the one proven
// SHOULD FAIL - proved for pred1 (x > 0), not for pred2 (x > 2)
proof fn test_logical_different_predicate() {
    let s = seq![1int, 2, 3];
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x > 2;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred1);
    // Try to claim filter with a different pred also gives identity
    assert(s.filter(pred2) == s);
}

// Test 5: Try to prove a stronger conclusion - filter with any pred is identity
// SHOULD FAIL - filter is only identity when ALL elements satisfy pred
proof fn test_logical_filter_always_identity() {
    let s = seq![1int, 2, 3];
    // Try to claim for an arbitrary pred without knowing if elements satisfy it
    assert(forall |pred: spec_fn(int) -> bool| s.filter(pred) == s);
}

}
