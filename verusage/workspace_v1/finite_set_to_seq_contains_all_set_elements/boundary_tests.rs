use vstd::prelude::*;

fn main() {}

verus! {

// ====== Target function signatures (copied from source) ======

#[verifier::external_body]
proof fn element_in_finite_set_exists_in_set_to_seq<A>(s: Set<A>, e: A)
    requires s.finite(), s.contains(e),
    ensures s.to_seq().contains(e),
{ unimplemented!() }

#[verifier::external_body]
proof fn element_in_seq_exists_in_original_finite_set<A>(s: Set<A>, e: A)
    requires s.finite(), s.to_seq().contains(e),
    ensures s.contains(e),
{ unimplemented!() }

pub proof fn finite_set_to_seq_contains_all_set_elements<A>(s: Set<A>)
    requires s.finite(),
    ensures forall |e: A| #[trigger] s.contains(e) <==> #[trigger] s.to_seq().contains(e)
{
    if s.len() != 0 {
        assert forall |e: A| #[trigger] s.contains(e) implies s.to_seq().contains(e) by {
            element_in_finite_set_exists_in_set_to_seq(s, e);
        }
        assert forall |e: A| #[trigger] s.to_seq().contains(e) implies s.contains(e) by {
            element_in_seq_exists_in_original_finite_set(s, e);
        }
    }
}

// ====== Boundary Tests ======

// Test 1: Call main lemma on an infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_infinite_set() {
    let s = Set::<int>::new(|x: int| true); // universal set, not finite
    finite_set_to_seq_contains_all_set_elements(s);
}

// Test 2: Call forward-direction axiom with element not in set (violates requires s.contains(e))
// SHOULD FAIL
proof fn test_boundary_element_not_in_set() {
    let s = Set::<int>::empty();
    element_in_finite_set_exists_in_set_to_seq(s, 42int);
}

// Test 3: Call reverse-direction axiom on infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_reverse_non_finite() {
    let s = Set::<int>::new(|x: int| true);
    element_in_seq_exists_in_original_finite_set(s, 0int);
}

// Test 4: Call forward-direction axiom on infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_forward_non_finite() {
    let s = Set::<int>::new(|x: int| true);
    element_in_finite_set_exists_in_set_to_seq(s, 0int);
}

// Test 5: Call main lemma and assert result about an element after violating finiteness
// SHOULD FAIL
proof fn test_boundary_derive_from_infinite() {
    let s = Set::<int>::new(|x: int| x >= 0); // infinite set of non-negatives
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().contains(1int));
}

}
