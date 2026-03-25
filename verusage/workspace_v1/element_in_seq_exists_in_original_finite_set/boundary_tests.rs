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

// ===== BOUNDARY TESTS =====

// Test 1: Empty set — to_seq() of empty set cannot contain any element
// Violates: s.to_seq().contains(e)
// SHOULD FAIL
proof fn test_boundary_empty_set() {
    let s = Set::<int>::empty();
    let e: int = 0;
    element_in_seq_exists_in_original_finite_set(s, e);
}

// Test 2: Finite set but element is NOT in the set (and hence not in to_seq)
// Violates: s.to_seq().contains(e)
// SHOULD FAIL
proof fn test_boundary_element_not_in_set() {
    let s = Set::<int>::empty().insert(1).insert(2);
    let e: int = 42;
    element_in_seq_exists_in_original_finite_set(s, e);
}

// Test 3: Infinite set — s.finite() does not hold
// Violates: s.finite()
// SHOULD FAIL
proof fn test_boundary_infinite_set() {
    let s = Set::<int>::new(|i: int| true);
    let e: int = 0;
    element_in_seq_exists_in_original_finite_set(s, e);
}

}
