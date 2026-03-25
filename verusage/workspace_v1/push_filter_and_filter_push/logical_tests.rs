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

// SHOULD FAIL: Two pushes commute under filter
// push order matters; filter preserves relative order,
// so push(a).push(b).filter != push(b).push(a).filter in general.
proof fn logical_push_commutativity_under_filter()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    let a: int = 1;
    let b: int = 2;
    push_filter_and_filter_push(s.push(a), pred, b);
    push_filter_and_filter_push(s.push(b), pred, a);
    assert(s.push(a).push(b).filter(pred) == s.push(b).push(a).filter(pred));
}

// SHOULD FAIL: Filter always produces non-empty result
// This is false for empty sequences with non-matching elements.
proof fn logical_filter_always_nonempty()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 100;
    let e: int = 1;
    push_filter_and_filter_push(s, pred, e);
    assert(s.push(e).filter(pred).len() > 0);
}

// SHOULD FAIL: Different predicates yield the same filter result
// Two different predicates should generally produce different results.
proof fn logical_different_predicates_same_result()
{
    let s: Seq<int> = Seq::empty().push(5).push(-3);
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x < 0;
    let e: int = 10;
    push_filter_and_filter_push(s, pred1, e);
    push_filter_and_filter_push(s, pred2, e);
    assert(s.push(e).filter(pred1) == s.push(e).filter(pred2));
}

// SHOULD FAIL: Double push-filter collapses to single filter
// Applying the lemma twice in sequence doesn't mean the intermediate push disappears.
proof fn logical_double_push_filter_collapse()
{
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    let a: int = 1;
    let b: int = 2;
    push_filter_and_filter_push(s, pred, a);
    push_filter_and_filter_push(s.push(a), pred, b);
    // Wrongly claim that pushing two elements and filtering equals filtering original
    assert(s.push(a).push(b).filter(pred) == s.filter(pred));
}

}
