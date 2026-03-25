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

// --- Behavioral Mutation Tests ---

// SHOULD FAIL: For seq [1, 2], map = x+10, result should be {11, 12}. Asserting 1 is in the result is wrong.
proof fn test_mutation_wrong_element_in_result() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x + 10);
    // Result should be {11, 12}, not contain 1
    assert(s.map_values(|x: int| x + 10).to_set().contains(1int)); // SHOULD FAIL
}

// SHOULD FAIL: Negate the postcondition for a concrete input — the sets should be equal, not unequal.
proof fn test_mutation_negated_postcondition() {
    let s: Seq<int> = Seq::empty().push(5int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x * 2);
    assert(s.map_values(|x: int| x * 2).to_set() != s.to_set().mk_map(|x: int| x * 2).values()); // SHOULD FAIL
}

// SHOULD FAIL: Using a different mapping function should produce a different result set.
// Asserting double equals triple's result is wrong.
proof fn test_mutation_wrong_function_equivalence() {
    let s: Seq<int> = Seq::empty().push(2int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x * 2);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| x * 3);
    // {4} != {6}
    assert(s.map_values(|x: int| x * 2).to_set() == s.map_values(|x: int| x * 3).to_set()); // SHOULD FAIL
}

// SHOULD FAIL: For seq [1, 2, 3], map = x*0 = 0 for all. Result is {0}.
// Asserting it contains 1 is wrong.
proof fn test_mutation_collapsing_map_wrong_member() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int).push(3int);
    map_values_to_set_eq_to_set_mk_map_values::<int, int>(s, |x: int| 0int);
    // All values collapse to 0, result set is {0}
    assert(s.map_values(|x: int| 0int).to_set().contains(1int)); // SHOULD FAIL
}

}
