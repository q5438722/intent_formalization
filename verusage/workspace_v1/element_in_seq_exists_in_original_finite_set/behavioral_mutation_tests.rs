#![allow(unused_imports)]
use vstd::prelude::*;
use vstd::set::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target function (copied from source) =====

proof fn element_in_seq_exists_in_original_finite_set<A>(s: Set<A>, e: A)
    requires s.finite(), s.to_seq().contains(e),
    ensures s.contains(e),
    decreases s.len()
{
    if s.len() != 0 {
        let x = s.choose();
        if x != e {
            element_in_seq_exists_in_original_finite_set(s.remove(x), e);
        }
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Negate the postcondition — assert !s.contains(e)
// The function guarantees s.contains(e), so its negation is contradictory
// SHOULD FAIL
proof fn test_mutation_negated_postcondition() {
    let s = Set::<int>::empty().insert(1).insert(2);
    let e: int = 1;
    assume(s.finite());
    assume(s.to_seq().contains(e));
    element_in_seq_exists_in_original_finite_set(s, e);
    assert(!s.contains(e));
}

// Test 2: Assert membership of a completely different element
// The function only guarantees s.contains(e) for the given e, not for 99
// SHOULD FAIL
proof fn test_mutation_wrong_element() {
    let s = Set::<int>::empty().insert(1).insert(2);
    let e: int = 1;
    assume(s.finite());
    assume(s.to_seq().contains(e));
    element_in_seq_exists_in_original_finite_set(s, e);
    assert(s.contains(99));
}

// Test 3: After removing e from the set, assert it is still contained
// s.remove(e) should NOT contain e
// SHOULD FAIL
proof fn test_mutation_element_after_removal() {
    let s = Set::<int>::empty().insert(1).insert(2);
    let e: int = 1;
    assume(s.finite());
    assume(s.to_seq().contains(e));
    element_in_seq_exists_in_original_finite_set(s, e);
    let s2 = s.remove(e);
    assert(s2.contains(e));
}

}
