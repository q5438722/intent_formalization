use vstd::prelude::*;

fn main() {}

verus!{

// ========== Original specification under test ==========

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

// ============================================================
// (1) BOUNDARY TESTS — Violate preconditions / edge cases
// ============================================================

// SHOULD FAIL: Empty sequence — filter of empty is empty, precondition unsatisfiable
proof fn test_boundary_empty_seq() {
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| x > 0;
    seq_filter_contains_implies_seq_contains(s, pred, 42);
}

// SHOULD FAIL: Element excluded by predicate — no element satisfies x > 10
proof fn test_boundary_element_excluded_by_pred() {
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 10;
    seq_filter_contains_implies_seq_contains(s, pred, 1);
}

// SHOULD FAIL: Element not in sequence at all — 99 ∉ s
proof fn test_boundary_element_not_in_seq() {
    let s: Seq<int> = Seq::empty().push(1).push(2).push(3);
    let pred = |x: int| x > 0;
    seq_filter_contains_implies_seq_contains(s, pred, 99);
}

// SHOULD FAIL: Always-false predicate — filter is always empty
proof fn test_boundary_always_false_pred() {
    let s: Seq<int> = Seq::empty().push(10).push(20).push(30);
    let pred = |x: int| false;
    seq_filter_contains_implies_seq_contains(s, pred, 10);
}

// SHOULD FAIL: Axiom on empty seq then assert contains — empty seq contains nothing
proof fn test_boundary_axiom_empty_seq_contains() {
    let s: Seq<int> = Seq::empty();
    let pred = |x: int| true;
    seq_filter_is_a_subset_of_original_seq::<int>(s, pred);
    assert(s.contains(0));
}

// ============================================================
// (2) BEHAVIORAL MUTATION TESTS — Mutate expected relations
// ============================================================

// SHOULD FAIL: Negated postcondition — containment in filter implies presence, not absence
proof fn test_mutation_negated_postcondition(s: Seq<int>, pred: spec_fn(int) -> bool, elt: int)
    requires s.filter(pred).contains(elt),
    ensures !s.contains(elt),
{
    seq_filter_contains_implies_seq_contains(s, pred, elt);
}

// SHOULD FAIL: General converse — s.contains(e) does NOT imply s.filter(pred).contains(e)
proof fn test_mutation_converse_direction()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool, e: int|
        s.contains(e) ==> s.filter(pred).contains(e),
{
}

// SHOULD FAIL: Filter preserves length — filtering generally removes elements
proof fn test_mutation_filter_preserves_length()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        s.filter(pred).len() == s.len(),
{
}

// SHOULD FAIL: Wrong element transfer — knowing elt1 ∈ filter does NOT prove elt2 ∈ s
proof fn test_mutation_wrong_element(s: Seq<int>, pred: spec_fn(int) -> bool, elt1: int, elt2: int)
    requires
        s.filter(pred).contains(elt1),
        elt1 != elt2,
    ensures s.contains(elt2),
{
    seq_filter_contains_implies_seq_contains(s, pred, elt1);
}

// SHOULD FAIL: Cross-predicate — filter(pred1).contains(e) ⇏ filter(pred2).contains(e)
proof fn test_mutation_cross_predicate()
    ensures forall |s: Seq<int>, pred1: spec_fn(int) -> bool, pred2: spec_fn(int) -> bool, e: int|
        s.filter(pred1).contains(e) ==> s.filter(pred2).contains(e),
{
}

// ============================================================
// (3) LOGICAL TESTS — Unintended properties / reasoning
// ============================================================

// SHOULD FAIL: Derive false — axiom should not be unsound
proof fn test_logical_derive_false()
    ensures false,
{
    let s: Seq<int> = Seq::empty().push(1);
    let pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq::<int>(s, pred);
}

// SHOULD FAIL: Stronger postcondition — filter containment does NOT pin index
proof fn test_logical_specific_index(s: Seq<int>, pred: spec_fn(int) -> bool, elt: int)
    requires
        s.filter(pred).contains(elt),
        s.len() > 0,
    ensures s[0] == elt,
{
    seq_filter_contains_implies_seq_contains(s, pred, elt);
}

// SHOULD FAIL: Uniqueness — filter can return more than one match
proof fn test_logical_uniqueness()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        s.filter(pred).len() <= 1,
{
}

// SHOULD FAIL: Filter equals identity — filter is NOT a no-op
proof fn test_logical_filter_is_identity()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        #[trigger] s.filter(pred) =~= s,
{
}

// SHOULD FAIL: Predicate-irrelevant length — different predicates give different filter lengths
proof fn test_logical_pred_irrelevant_to_length()
    ensures forall |s: Seq<int>, pred1: spec_fn(int) -> bool, pred2: spec_fn(int) -> bool|
        #[trigger] s.filter(pred1).len() == #[trigger] s.filter(pred2).len(),
{
}

}
