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

// SHOULD FAIL: Assert commutativity holds WITHOUT calling the lemma
// The property should not be derivable from background theory alone (filter is opaque).
// Tests whether Verus can magically derive commutativity without the proof.
proof fn logical_commutativity_without_lemma()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x * 2;
    let pred_on_mapped = |y: int| y > 0;
    // Do NOT call the lemma — just assert the postcondition directly
    assert(s.map_values(map_fn).filter(pred_on_mapped) == s.filter(pred).map_values(map_fn));
}

// SHOULD FAIL: Different predicates yield the same filter result
// pred1 keeps positives, pred2 keeps negatives — results should differ.
// Tests whether the spec allows unintended predicate equivalence.
proof fn logical_different_preds_same_filter()
{
    let s: Seq<int> = Seq::empty().push(1).push(-2).push(3);
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x < 0;
    let map_fn = |x: int| x;
    let pred_on_mapped1 = |y: int| y > 0;
    let pred_on_mapped2 = |y: int| y < 0;
    commutativity_of_seq_map_and_filter(s, pred1, pred_on_mapped1, map_fn);
    commutativity_of_seq_map_and_filter(s, pred2, pred_on_mapped2, map_fn);
    // Wrongly claim two different filters produce the same result
    assert(s.filter(pred1) == s.filter(pred2));
}

// SHOULD FAIL: Extend result to a larger sequence without re-proving precondition
// The lemma is only called for s, but we assert the property for s.push(-1).
// Tests whether the spec accidentally applies beyond its scope.
proof fn logical_extend_to_pushed_element()
{
    let s: Seq<int> = Seq::empty().push(1);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x * 2;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Extend to s2 = [1, -1] without calling the lemma again
    let s2: Seq<int> = s.push(-1);
    assert(s2.map_values(map_fn).filter(pred_on_mapped) == s2.filter(pred).map_values(map_fn));
}

// SHOULD FAIL: The spec does not imply map is injective
// map(x) = x*x maps both 1 and -1 to 1, so map_values[0] == map_values[1].
// Tests whether the spec accidentally entails injectivity of the map function.
proof fn logical_map_not_injective()
{
    let s: Seq<int> = Seq::empty().push(1).push(-1);
    let pred = |x: int| true;
    let map_fn = |x: int| x * x;
    let pred_on_mapped = |y: int| true;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
    // Wrongly claim map is injective: different inputs must give different outputs
    assert(s.map_values(map_fn)[0] != s.map_values(map_fn)[1]);
}

}
