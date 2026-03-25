use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemma under test
#[verifier::spinoff_prover]
pub proof fn lemma_map_set_singleton_auto<A, B>()
ensures
    forall |x: A, f: spec_fn(A) -> B| #[trigger] set![x].map(f) == set![f(x)],
{
    assert forall |x: A, f: spec_fn(A) -> B| #[trigger] set![x].map(f) =~= set![f(x)] by {
        assert(set![x].contains(x));
    }
}

// === BOUNDARY TESTS ===

// Boundary Test 1: Mapping over the empty set should NOT produce a singleton.
// SHOULD FAIL
proof fn test_boundary_empty_set_map_not_singleton() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(Set::<int>::empty().map(|x: int| x + 1) == set![1int]);
}

// Boundary Test 2: Mapping over a two-element set should NOT equal a one-element set.
// SHOULD FAIL
proof fn test_boundary_two_element_set_map_collapses() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![1int, 2int].map(|x: int| x) == set![1int]);
}

// Boundary Test 3: Mapping a singleton should NOT produce an empty set.
// SHOULD FAIL
proof fn test_boundary_singleton_map_not_empty() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![5int].map(|x: int| 0int) == Set::<int>::empty());
}

}
