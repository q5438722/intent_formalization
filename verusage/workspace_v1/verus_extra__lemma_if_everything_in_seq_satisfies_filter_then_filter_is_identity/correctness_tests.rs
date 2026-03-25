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
// BOUNDARY TESTS (B1-B5): Violate preconditions
// ============================================================

// B1: Some elements do NOT satisfy pred
// SHOULD FAIL
proof fn test_boundary_not_all_elements_satisfy_pred() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 2;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// B2: No elements satisfy pred
// SHOULD FAIL
proof fn test_boundary_no_elements_satisfy_pred() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// B3: Single element fails pred
// SHOULD FAIL
proof fn test_boundary_single_element_fails_pred() {
    let s = seq![0int];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// B4: Only last element fails pred
// SHOULD FAIL
proof fn test_boundary_last_element_fails() {
    let s = seq![1int, 2, 3, -1];
    let pred = |x: int| x >= 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// B5: Only first element fails pred
// SHOULD FAIL
proof fn test_boundary_first_element_fails() {
    let s = seq![-1int, 1, 2, 3];
    let pred = |x: int| x >= 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
}

// ============================================================
// BEHAVIORAL MUTATION TESTS (M1-M5): Valid inputs, wrong claims
// ============================================================

// M1: Assert filter != s (negates ensures)
// SHOULD FAIL
proof fn test_mutation_filter_not_equal_to_original() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred) != s);
}

// M2: Assert filter produces empty sequence
// SHOULD FAIL
proof fn test_mutation_filter_gives_empty() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred).len() == 0);
}

// M3: Assert filter produces shorter sequence
// SHOULD FAIL
proof fn test_mutation_filter_shorter_length() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred).len() < s.len());
}

// M4: Assert filter equals a reordered sequence
// SHOULD FAIL
proof fn test_mutation_filter_equals_wrong_seq() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    let wrong = seq![3int, 2, 1];
    assert(s.filter(pred) =~= wrong);
}

// M5: Assert filter changes an element value
// SHOULD FAIL
proof fn test_mutation_filter_changes_element() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(s.filter(pred)[0] == 99);
}

// ============================================================
// LOGICAL TESTS (L1-L5): Properties not entailed by the spec
// ============================================================

// L1: Assert conclusion without calling the lemma
// SHOULD FAIL
proof fn test_logical_conclusion_without_lemma() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    assert(s.filter(pred) == s);
}

// L2: Derive that pred holds for values outside the sequence
// SHOULD FAIL
proof fn test_logical_pred_universally_true() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred);
    assert(pred(-5));
}

// L3: Apply conclusion to a different sequence
// SHOULD FAIL
proof fn test_logical_cross_sequence_misuse() {
    let s1 = seq![1int, 2, 3];
    let s2 = seq![1int, -1, 3];
    let pred = |x: int| x > 0;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s1, pred);
    assert(s2.filter(pred) == s2);
}

// L4: Apply conclusion with a different predicate
// SHOULD FAIL
proof fn test_logical_different_predicate() {
    let s = seq![1int, 2, 3];
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x > 2;
    lemma_if_everything_in_seq_satisfies_filter_then_filter_is_identity(s, pred1);
    assert(s.filter(pred2) == s);
}

// L5: Claim filter is identity for ANY predicate
// SHOULD FAIL
proof fn test_logical_filter_always_identity() {
    let s = seq![1int, 2, 3];
    assert(forall |pred: spec_fn(int) -> bool| s.filter(pred) == s);
}

}
