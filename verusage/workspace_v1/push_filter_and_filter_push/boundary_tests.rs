use vstd::prelude::*;

fn main() {}

verus! {

// Trusted specification under test (body replaced with admit)
pub proof fn push_filter_and_filter_push<A>(s: Seq<A>, pred: spec_fn(A) -> bool, e: A)
    ensures
        pred(e) ==> s.push(e).filter(pred) == s.filter(pred).push(e),
        !pred(e) ==> s.push(e).filter(pred) == s.filter(pred),
{
    admit();
}

// SHOULD FAIL: Non-matching element still increases filter length
// When pred(e) is false, filter(pred) should NOT grow. This wrongly claims it does.
proof fn boundary_non_matching_increases_length()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    let e: int = -1;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred).len() == s.filter(pred).len() + 1);
}

// SHOULD FAIL: Matching element doesn't increase filter length
// When pred(e) is true, filter length should grow by 1. This wrongly claims it stays the same.
proof fn boundary_matching_no_length_change()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    let e: int = 5;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred).len() == s.filter(pred).len());
}

// SHOULD FAIL: Filter of push equals push (filter has no effect)
// Claim that filtering doesn't remove anything, even with mixed-sign elements.
proof fn boundary_filter_is_identity()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred = |x: int| x > 0;
    let e: int = 4;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred) == s.push(e));
}

}
