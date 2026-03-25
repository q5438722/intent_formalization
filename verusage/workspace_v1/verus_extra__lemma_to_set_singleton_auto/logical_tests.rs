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

// === LOGICAL TESTS ===
// These test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: the lemma does NOT entail that a two-element seq converts to a singleton set
proof fn test_logical_multi_element_seq_to_singleton() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int, 2int].to_set() == set![1int]);
}

// SHOULD FAIL: wrong cardinality — the singleton set has exactly 1 element, not 2
proof fn test_logical_wrong_cardinality() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set().len() == 2);
}

// SHOULD FAIL: subset of unrelated set — {1} is NOT a subset of {2, 3}
proof fn test_logical_unrelated_subset() {
    lemma_to_set_singleton_auto::<int>();
    assert(seq![1int].to_set().subset_of(set![2int, 3int]));
}

}
