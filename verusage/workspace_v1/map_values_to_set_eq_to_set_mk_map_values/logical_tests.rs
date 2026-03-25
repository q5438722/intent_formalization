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

// --- Logical Tests ---

// SHOULD FAIL: The spec does NOT guarantee that map_values preserves cardinality.
// A non-injective map collapses distinct elements, so |result set| < |seq|.
proof fn test_logical_size_preservation_claim() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int).push(3int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| 0int);
    // All map to 0 => result set size is 1, but seq length is 3
    assert(s.map_values(|x: int| 0int).to_set().len() == s.len()); // SHOULD FAIL
}

// SHOULD FAIL: The spec does NOT guarantee map_values(identity).to_set() == to_set() when the
// map function is not actually the identity — using a non-identity and claiming identity behavior.
proof fn test_logical_non_identity_treated_as_identity() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x + 100);
    // map(x) = x+100 is NOT identity, so mapped set != original set
    assert(s.map_values(|x: int| x + 100).to_set() == s.to_set()); // SHOULD FAIL
}

// SHOULD FAIL: Chaining the push axiom should NOT allow deriving membership of elements
// that were never pushed. Testing that repeated axiom application doesn't create "phantom" elements.
proof fn test_logical_axiom_chain_phantom_element() {
    let s0: Seq<int> = Seq::empty();
    let s1 = s0.push(10int);
    let s2 = s1.push(20int);
    push_to_set_eq_to_set_insert::<int>(s0, 10int);
    push_to_set_eq_to_set_insert::<int>(s1, 20int);
    // s2.to_set() == {10, 20}, should NOT contain 30
    assert(s2.to_set().contains(30int)); // SHOULD FAIL
}

// SHOULD FAIL: The postcondition is about set equality, NOT about injectivity of the map.
// The spec should not allow proving that a constant map produces distinct values.
proof fn test_logical_false_injectivity() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| 42int);
    // Constant map: all go to 42. Result set = {42}, size = 1
    // Claiming size equals original seq length (2) is false
    assert(s.to_set().mk_map(|x: int| 42int).values().len() == 2); // SHOULD FAIL
}

}
