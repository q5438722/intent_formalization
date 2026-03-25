use vstd::prelude::*;

fn main() {}

verus! {

proof fn empty_filter_implies_seq_pred_false_on_all_elements<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires s.filter(pred).len() == 0,
    ensures forall |e: A| #![auto] s.contains(e) ==> !pred(e)
    decreases s.len()
{
    if s.len() != 0 {
        let subseq = s.drop_last();
        assert(!pred(s.last())) by {
            reveal(Seq::filter);
            assert(s.filter(pred) == {
                if pred(s.last()) {
                    subseq.filter(pred).push(s.last())
                } else {
                    subseq.filter(pred)
                }
            })
        }
        assert(s.filter(pred) == subseq.filter(pred)) by {
            reveal(Seq::filter);
            assert(!pred(s.last()));
        }
        empty_filter_implies_seq_pred_false_on_all_elements(s.drop_last(), pred);
        assert forall |e: A| #![auto] s.contains(e) ==> !pred(e) by {
            assert(forall |i: int| 0 <= i < subseq.len() ==> (subseq.contains(#[trigger] subseq[i]) ==> !pred(subseq[i])));
            assert(forall |i: int| 0 <= i < subseq.len() ==> s[i] == subseq[i]);
        }
    }
}

// Behavioral Mutation Test 1: Flip negation — assert pred(e) instead of !pred(e)
// Valid precondition, but mutated postcondition
// SHOULD FAIL
proof fn mutation_test_flip_negation(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    assert(forall |e: int| #![auto] s.contains(e) ==> pred(e));
}

// Behavioral Mutation Test 2: Change quantifier — assert exists element satisfying pred
// Valid precondition, but mutated from forall-not to exists
// SHOULD FAIL
proof fn mutation_test_exists_pred_true(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    assert(exists |e: int| s.contains(e) && pred(e));
}

// Behavioral Mutation Test 3: Assert pred is true on a specific element (first)
// Valid precondition, but asserts pred(s[0]) instead of !pred(s[0])
// SHOULD FAIL
proof fn mutation_test_first_element_pred_true(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    assert(pred(s[0]));
}

}
