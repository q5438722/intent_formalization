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

// SHOULD FAIL: Swap postconditions — when pred(e), claim push is dropped
// The spec says pred(e) ==> result == s.filter(pred).push(e).
// This wrongly claims pred(e) ==> result == s.filter(pred) (no push).
proof fn mutation_pred_true_no_push()
{
    let s: Seq<int> = Seq::empty().push(1).push(2);
    let pred = |x: int| x > 0;
    let e: int = 3;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred) == s.filter(pred));
}

// SHOULD FAIL: Swap postconditions — when !pred(e), claim push is included
// The spec says !pred(e) ==> result == s.filter(pred).
// This wrongly claims !pred(e) ==> result == s.filter(pred).push(e).
proof fn mutation_pred_false_with_push()
{
    let s: Seq<int> = Seq::empty().push(1).push(2);
    let pred = |x: int| x > 0;
    let e: int = -1;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred) == s.filter(pred).push(e));
}

// SHOULD FAIL: Wrong element pushed — when pred(e), push different element
// The spec says pred(e) ==> result == s.filter(pred).push(e).
// This wrongly claims result == s.filter(pred).push(e + 1).
proof fn mutation_wrong_element_pushed()
{
    let s: Seq<int> = Seq::empty().push(10);
    let pred = |x: int| x > 0;
    let e: int = 5;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred) == s.filter(pred).push((e + 1) as int));
}

// SHOULD FAIL: Filter result equals unfiltered original seq
// Mutate the RHS to be just s (not s.filter(pred))
proof fn mutation_filter_equals_original()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred = |x: int| x > 0;
    let e: int = -5;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred) == s);
}

}
