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

// ===== LOGICAL TESTS =====

// Test 1: Converse direction — from s.contains(e), derive s.to_seq().contains(e)
// The function only proves: to_seq contains ⟹ set contains
// The converse is NOT guaranteed by this function
// SHOULD FAIL
proof fn test_logical_converse() {
    let s = Set::<int>::empty().insert(1).insert(2);
    assume(s.finite());
    let e: int = 1;
    assert(s.to_seq().contains(e));
}

// Test 2: Determinism — assert specific element ordering in to_seq()
// to_seq() does not guarantee any particular ordering of elements
// SHOULD FAIL
proof fn test_logical_seq_ordering() {
    let s = Set::<int>::empty().insert(1).insert(2);
    assume(s.finite());
    let seq = s.to_seq();
    assume(seq.len() >= 1);
    assert(seq[0] == 1);
}

// Test 3: Universal generalization — a single-element proof should not
// establish the property for ALL elements without repeated invocations
// SHOULD FAIL
proof fn test_logical_universal_from_single() {
    let s = Set::<int>::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    assume(s.finite());
    assume(s.to_seq().contains(e));
    element_in_seq_exists_in_original_finite_set(s, e);
    // Only proved s.contains(1), try to derive universal property
    assert(forall |x: int| s.to_seq().contains(x) ==> s.contains(x));
}

}
