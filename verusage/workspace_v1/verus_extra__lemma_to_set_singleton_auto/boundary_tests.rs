use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// Original specification
#[verifier::spinoff_prover]
pub proof fn lemma_to_set_singleton_auto<A>()
ensures
    forall |x: A| #[trigger] seq![x].to_set() == set![x],
{
    assert forall |x: A| #[trigger] seq![x].to_set() =~= set![x] by {
        assert(seq![x][0] == x);
    }
}

// === BOUNDARY TESTS ===
// These violate the semantic boundaries of the specification.

// SHOULD FAIL: singleton seq to_set is NOT the empty set
proof fn test_boundary_singleton_to_empty_set() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![0int].to_set() == Set::<int>::empty());
}

// SHOULD FAIL: singleton seq to_set does NOT equal a two-element set
proof fn test_boundary_singleton_equals_two_element_set() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set() == set![1int, 2int]);
}

// SHOULD FAIL: the to_set of seq![5] does NOT contain element 3
proof fn test_boundary_contains_absent_element() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![5int].to_set().contains(3int));
}

}
