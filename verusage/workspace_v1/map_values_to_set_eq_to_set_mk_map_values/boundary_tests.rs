use vstd::prelude::*;

fn main() {}

verus! {

// --- Spec under test (trusted signatures) ---

#[verifier::external_body]
pub proof fn push_to_set_eq_to_set_insert<A>(s: Seq<A>, e: A)
    ensures s.push(e).to_set() == s.to_set().insert(e)
{ unimplemented!() }

#[verifier::external_body]
pub proof fn map_values_to_set_eq_to_set_mk_map_values<A, B>(s: Seq<A>, map: spec_fn(A) -> B)
    ensures s.map_values(map).to_set() == s.to_set().mk_map(map).values(),
{ unimplemented!() }

// --- Boundary Tests ---

// SHOULD FAIL: Empty sequence's mapped-to-set must be empty; asserting it contains an element is invalid.
proof fn test_boundary_empty_seq_contains_element() {
    let s: Seq<int> = Seq::empty();
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x + 1);
    assert(s.map_values(|x: int| x + 1).to_set().contains(1int)); // SHOULD FAIL
}

// SHOULD FAIL: Push axiom establishes {5} for empty().push(5); asserting it contains 10 is invalid.
proof fn test_boundary_push_axiom_wrong_membership() {
    let s: Seq<int> = Seq::empty();
    push_to_set_eq_to_set_insert::<int>(s, 5int);
    assert(s.push(5int).to_set().contains(10int)); // SHOULD FAIL
}

// SHOULD FAIL: A singleton seq maps to a singleton set; its cardinality cannot exceed 1.
proof fn test_boundary_singleton_set_oversized() {
    let s: Seq<int> = Seq::empty().push(3int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x * 2);
    assert(s.map_values(|x: int| x * 2).to_set().len() > 1); // SHOULD FAIL
}

// SHOULD FAIL: Calling lemma with no preconditions on a length-0 seq should not make to_set non-empty.
proof fn test_boundary_empty_map_nonempty_values() {
    let s: Seq<int> = Seq::empty();
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| 0int);
    assert(s.to_set().mk_map(|x: int| 0int).values().len() > 0); // SHOULD FAIL
}

}
