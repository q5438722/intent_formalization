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

// Boundary Test 1: Filter is non-empty — violates precondition (filter len > 0 vs required == 0)
// SHOULD FAIL
proof fn boundary_test_nonempty_filter(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
        s.filter(pred).len() > 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
}

// Boundary Test 2: Filter length is exactly 1 — off-by-one boundary of == 0
// SHOULD FAIL
proof fn boundary_test_filter_len_one(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() == 1,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
}

// Boundary Test 3: Filter length equals sequence length (all elements match)
// SHOULD FAIL
proof fn boundary_test_all_elements_match(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
        s.filter(pred).len() == s.len(),
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
}

}
