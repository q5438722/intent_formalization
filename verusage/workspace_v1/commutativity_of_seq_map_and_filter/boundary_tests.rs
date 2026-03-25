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

// SHOULD FAIL: Predicates completely disagree on all elements
// pred(s[i]) = true but pred_on_mapped(map(s[i])) = false for every i.
// The precondition requires pred(s[i]) == pred_on_mapped(map(s[i])), which is violated.
proof fn boundary_predicates_fully_disagree()
{
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x;
    let pred_on_mapped = |y: int| y < 0;
    // pred(1)=true, pred_on_mapped(1)=false → DISAGREE for all elements
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
}

// SHOULD FAIL: Predicates disagree on exactly one element
// s[0]: pred(1)=true, map(1)=3, pred_on_mapped(3)=true  → agree
// s[1]: pred(-1)=false, map(-1)=1, pred_on_mapped(1)=true → DISAGREE
proof fn boundary_partial_disagreement()
{
    let s: Seq<int> = Seq::empty().push(1).push(-1);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| x + 2;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
}

// SHOULD FAIL: Map negates all values, making predicate-compatibility impossible
// s[0]=5: pred(5)=true, map(5)=-5, pred_on_mapped(-5)=false → DISAGREE
proof fn boundary_map_negates_values()
{
    let s: Seq<int> = Seq::empty().push(5);
    let pred = |x: int| x > 0;
    let map_fn = |x: int| -x;
    let pred_on_mapped = |y: int| y > 0;
    commutativity_of_seq_map_and_filter(s, pred, pred_on_mapped, map_fn);
}

}
