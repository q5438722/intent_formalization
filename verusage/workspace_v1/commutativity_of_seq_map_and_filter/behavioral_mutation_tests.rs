use vstd::prelude::*;

fn main() {}

verus! {

// Trusted specification under test (body replaced with admit)
pub proof fn commutativity_of_seq_map_and_filter<A, B>(s: Seq<A>, pred: spec_fn(A) -> bool, pred_on_mapped: spec_fn(B) -> bool, map: spec_fn(A) -> B)
    requires forall |i: int| 0 <= i < s.len() ==> #[trigger] pred(s[i]) == #[trigger] pred_on_mapped(map(s[i])),
    ensures s.map_values(map).filter(pred_on_mapped) == s.filter(pred).map_values(map),
{
    admit();
}

// SHOULD FAIL: Assert filter has no effect — map-then-filter equals just map
// With mixed-sign elements, filtering SHOULD remove negatives.
// Mutates postcondition by dropping the filter on the RHS.
proof fn mutation_filter_is_noop()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Wrongly claim that filtering does nothing (removes filter from RHS)
    assert(s.map_values(map_fn).filter(pred_on_mapped) == s.map_values(map_fn));
}

// SHOULD FAIL: Assert filtered result has same length as original
// Element -2 should be filtered out, reducing length from 3 to 2.
// Mutates the length relationship.
proof fn mutation_length_preserved()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Wrongly claim filtered result has same length as original
    assert(s.filter(pred).map_values(map_fn).len() == s.len());
}

// SHOULD FAIL: Assert commutativity holds with a DIFFERENT map function
// The spec establishes commutativity for map_fn, not other_map.
// Mutates the map function on one side.
proof fn mutation_different_map_same_result()
{
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x * 2;
    let other_map = |x: int| x * 3;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Wrongly assert the result equals using a completely different map
    assert(s.map_values(map_fn).filter(pred_on_mapped) == s.filter(pred).map_values(other_map));
}

// SHOULD FAIL: Assert commutativity result is the empty sequence
// All elements are positive so nothing is filtered out; result should be non-empty.
// Mutates the expected result to empty.
proof fn mutation_result_is_empty()
{
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x * 2;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Wrongly claim the result is empty
    assert(s.filter(pred).map_values(map_fn) == Seq::<int>::empty());
}

}
