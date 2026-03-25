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

// === BEHAVIORAL MUTATION TEST 1 ===
// Valid inputs (pred false on all elements), but mutate expected output:
// assert filter length is 1 instead of 0.
// The postcondition guarantees len() == 0, so asserting == 1 should fail.
// SHOULD FAIL
proof fn behavioral_mutation_test_1_filter_len_is_one() {
    let s: Seq<int> = seq![1, 2, 3];
    let pred = |x: int| x > 10;
    // pred is false on all elements of s (1, 2, 3 are all <= 10)
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Mutated assertion: filter length should be 1 (wrong)
    assert(s.filter(pred).len() == 1);
}

// === BEHAVIORAL MUTATION TEST 2 ===
// Valid inputs, but assert filter length is strictly greater than 0.
// The postcondition guarantees len() == 0, so > 0 should be contradictory.
// SHOULD FAIL
proof fn behavioral_mutation_test_2_filter_len_gt_zero() {
    let s: Seq<int> = seq![5, 10, 15];
    let pred = |x: int| x < 0;
    // pred is false on all elements (all positive)
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Mutated assertion: filter should be non-empty (wrong)
    assert(s.filter(pred).len() > 0);
}

// === BEHAVIORAL MUTATION TEST 3 ===
// Valid inputs, but negate the postcondition: assert filter length != 0.
// Directly contradicts the ensures clause.
// SHOULD FAIL
proof fn behavioral_mutation_test_3_filter_len_not_zero() {
    let s: Seq<int> = seq![100];
    let pred = |x: int| x == 0;
    // pred is false on element 100
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Negated postcondition: filter is not empty (wrong)
    assert(s.filter(pred).len() != 0);
}

}
