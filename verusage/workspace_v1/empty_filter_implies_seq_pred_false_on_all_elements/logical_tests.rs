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

// Logical Test 1: Empty filter does NOT imply the sequence itself is empty
// The spec only says pred is false on all elements, not that s has no elements
// SHOULD FAIL
proof fn logical_test_empty_filter_implies_empty_seq(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    assert(s.len() == 0);
}

// Logical Test 2: Conclusion about s1 does NOT transfer to a different sequence s2
// The spec scopes !pred to elements of the argument sequence only
// SHOULD FAIL
proof fn logical_test_cross_sequence_transfer(s1: Seq<int>, s2: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s1.filter(pred).len() == 0,
        s2.len() > 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s1, pred);
    assert(forall |e: int| #![auto] s2.contains(e) ==> !pred(e));
}

// Logical Test 3: !pred on elements of s does NOT generalize to all integers
// The spec guarantees !pred only for elements contained in s
// SHOULD FAIL
proof fn logical_test_global_pred_false(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    assert(forall |e: int| #![trigger pred(e)] !pred(e));
}

}
