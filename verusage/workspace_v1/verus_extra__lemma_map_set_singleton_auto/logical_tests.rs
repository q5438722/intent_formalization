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

// === LOGICAL TESTS ===

// Logical Test 1: Injectivity — the lemma does NOT imply that mapped functions are injective.
// If f(a) == f(b), we should NOT be able to conclude a == b.
// SHOULD FAIL
proof fn test_logical_injectivity_not_implied() {
    lemma_map_set_singleton_auto::<int, int>();
    let f = |x: int| 0int;  // constant function
    assert(set![1int].map(f) == set![2int].map(f));
    assert(1int == 2int);
}

// Logical Test 2: The lemma should NOT imply that mapping over a two-element set
// collapses to a singleton when using identity.
// SHOULD FAIL
proof fn test_logical_multi_element_map_is_singleton() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![1int, 2int].map(|x: int| x) == set![1int]);
}

// Logical Test 3: The lemma should NOT imply reverse mapping.
// set![6].map(|x| x+1) == {7}, NOT {5}.
// SHOULD FAIL
proof fn test_logical_no_reverse_mapping() {
    lemma_map_set_singleton_auto::<int, int>();
    assert(set![6int].map(|x: int| x + 1) == set![5int]);
}

}
