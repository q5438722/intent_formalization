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

// === BEHAVIORAL MUTATION TESTS ===

// Mutation Test 1: Wrong result value.
// set![3].map(|x| x + 1) should be {4}, NOT {5}.
// SHOULD FAIL
proof fn test_mutation_wrong_result_value() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![3int].map(|x: int| x + 1) == set![5int]);
}

// Mutation Test 2: Result is the original set (map is not identity in general).
// set![7].map(|x| x + 1) should be {8}, NOT {7}.
// SHOULD FAIL
proof fn test_mutation_result_equals_input() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![7int].map(|x: int| x + 1) == set![7int]);
}

// Mutation Test 3: Two different functions produce the same mapped set on input 3.
// set![3].map(|x| x + 1) == {4} but set![3].map(|x| x * 2) == {6}.
// SHOULD FAIL
proof fn test_mutation_different_functions_same_result() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![3int].map(|x: int| x + 1) == set![3int].map(|x: int| x * 2));
}

}
