use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemma (copied from source) =====
#[verifier::spinoff_prover]
pub proof fn lemma_set_map_insert<A, B>(s: Set<A>, f: spec_fn(A) -> B, x: A)
    ensures s.insert(x).map(f) == s.map(f).insert(f(x))
{
    assert_sets_equal!(s.insert(x).map(f) == s.map(f).insert(f(x)), y => {
        if y == f(x) {
            assert(s.insert(x).contains(x));
        } else {
            if s.insert(x).map(f).contains(y) {
                let x0 = choose |x0| s.contains(x0) && y == f(x0);
                assert(s.map(f).contains(y));
            } else {
                if s.map(f).insert(f(x)).contains(y) {
                    let x0 = choose |x0| s.contains(x0) && y == f(x0);
                    assert(s.map(f).contains(y));
                    assert(s.insert(x).contains(x0));
                }
            }
        }
    });
}

// ===== LOGICAL TESTS =====

// Test 1: Map injectivity — assert that a non-injective function preserves distinct elements
// f(x) = x % 2 maps both 0 and 2 to 0. The mapped set should NOT contain 2.
// The lemma gives no injectivity guarantee; this should NOT be provable.
// SHOULD FAIL
proof fn test_logical_map_not_injective()
{
    let s = Set::<int>::empty().insert(0int);
    let f = |x: int| (x % 2) as int;
    lemma_set_map_insert::<int, int>(s, f, 2int);
    // s.insert(2).map(f) = {0, 2}.map(|x| x%2) = {0}
    // Falsely claim 2 is in the mapped set
    assert(s.insert(2int).map(f).contains(2int));
}

// Test 2: Reverse mapping existence — the lemma does NOT guarantee preimage uniqueness
// or that arbitrary values appear in the mapped set.
// Try to claim that a value NOT in the range of f appears in the mapped set.
// SHOULD FAIL
proof fn test_logical_phantom_element_in_map()
{
    let s = Set::<int>::empty().insert(0int).insert(1int);
    let f = |x: int| (x + 10) as int;
    lemma_set_map_insert::<int, int>(s, f, 2int);
    // s.insert(2) = {0, 1, 2}
    // s.insert(2).map(f) = {10, 11, 12}
    // Falsely claim 0 is in the mapped set (it's not in range of f over this domain)
    assert(s.insert(2int).map(f).contains(0int));
}

// Test 3: Stronger cardinality claim — the lemma says nothing about set sizes.
// A non-injective map reduces cardinality. Try to claim the mapped set is a superset.
// Specifically: try to derive that mapping is injective (it shouldn't be for constant f).
// SHOULD FAIL
proof fn test_logical_map_preserves_distinctness()
{
    let s = Set::<int>::empty().insert(0int).insert(1int);
    let f = |x: int| 42int;
    lemma_set_map_insert::<int, int>(s, f, 2int);
    // s.insert(2) = {0, 1, 2}, mapped with const 42 gives {42}
    // Falsely assert that the mapped set contains at least two distinct values
    assert(s.insert(2int).map(f).contains(42int) && s.insert(2int).map(f).contains(0int));
}

// Test 4: Cross-function misuse — use the lemma with one function but assert
// a property about a different function applied to the same set.
// The lemma should not transfer between different functions.
// SHOULD FAIL
proof fn test_logical_cross_function_transfer()
{
    let s = Set::<int>::empty();
    let f = |x: int| (x + 1) as int;
    let g = |x: int| (x * 2) as int;
    lemma_set_map_insert::<int, int>(s, f, 3int);
    // Lemma gives: s.insert(3).map(f) == s.map(f).insert(f(3)) == {4}
    // Try to use this to conclude something about g:
    assert(s.insert(3int).map(g) =~= s.map(g).insert(f(3int)));
    // g(3) = 6, but f(3) = 4, so s.map(g).insert(4) != s.insert(3).map(g) = {6}
}

}
