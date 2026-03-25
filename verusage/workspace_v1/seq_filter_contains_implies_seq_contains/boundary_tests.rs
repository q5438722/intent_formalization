use vstd::prelude::*;

fn main() {}

verus!{

// ---- Original specification under test ----

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{ unimplemented!() }

pub proof fn seq_filter_contains_implies_seq_contains<A>(s: Seq<A>, pred: spec_fn(A) -> bool, elt: A)
    requires s.filter(pred).contains(elt),
    ensures s.contains(elt)
{
    seq_filter_is_a_subset_of_original_seq(s, pred);
}

// ---- Boundary Tests: Violate preconditions ----

// SHOULD FAIL: Empty sequence — filter of empty is empty, no element is contained
proof fn test_boundary_empty_seq() {
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    seq_filter_contains_implies_seq_contains(s, pred, 42);
}

// SHOULD FAIL: Element not satisfying predicate — filter excludes it
proof fn test_boundary_element_excluded_by_pred() {
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 10;
    // No element in s satisfies x > 10, so filter is empty
    seq_filter_contains_implies_seq_contains(s, pred, 1);
}

// SHOULD FAIL: Element not in sequence at all — cannot be in filtered result
proof fn test_boundary_element_not_in_seq() {
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 0;
    // 99 is not in s, so not in s.filter(pred) either
    seq_filter_contains_implies_seq_contains(s, pred, 99);
}

// SHOULD FAIL: Always-false predicate — filter returns empty sequence
proof fn test_boundary_always_false_pred() {
    let s: Seq<int> = Seq::empty().push(10).push(20).push(30);
    let pred = |x: int| false;
    seq_filter_contains_implies_seq_contains(s, pred, 10);
}

// SHOULD FAIL: Call axiom on empty seq, then assert it contains an element
proof fn test_boundary_axiom_empty_seq_contains() {
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| true;
    seq_filter_is_a_subset_of_original_seq::<int>(s, pred);
    assert(s.contains(0));
}

}
