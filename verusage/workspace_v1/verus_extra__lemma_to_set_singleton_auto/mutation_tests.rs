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

// === BEHAVIORAL MUTATION TESTS ===
// These mutate expected outputs/relations to check incorrect behaviors are rejected.

// SHOULD FAIL: swapped element — seq![1].to_set() is NOT set![2]
proof fn test_mutation_swapped_element() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set() == set![2int]);
}

// SHOULD FAIL: negated equality — seq![1].to_set() IS equal to set![1]
proof fn test_mutation_negated_equality() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set() != set![1int]);
}

// SHOULD FAIL: extra element — seq![1].to_set() does NOT contain 99
proof fn test_mutation_extra_element_in_result() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set().contains(99int));
}

}
