use vstd::prelude::*;

fn main() {}

verus! {

// ===== Target function (copied from source) =====

pub proof fn map_values_weakens_no_duplicates<A, B>(s: Seq<A>, map: spec_fn(A) -> B)
    requires s.map_values(map).no_duplicates()
    ensures s.no_duplicates()
{
    assert forall |i, j| 0 <= i < s.len() && 0 <= j < s.len() && i != j implies s[i] != s[j] by {
        if s[i] == s[j] {
            assert(s.map_values(map)[i] == s.map_values(map)[j]);
            assert(false);
        }
    }
}

// ===== LOGICAL TESTS =====

// Test 1: Try to derive no_duplicates for an arbitrary sequence
// without any precondition about the mapped sequence.
// The spec only guarantees no_duplicates WHEN the mapped seq has no dups.
// SHOULD FAIL
proof fn test_logical_no_dup_without_precondition(s: Seq<int>) {
    // No precondition at all — cannot conclude no_duplicates
    assert(s.no_duplicates());
}

// Test 2: Try to derive that the map function must be injective.
// The spec does not guarantee injectivity of map; it only says
// if mapped values are unique then original values are unique.
// SHOULD FAIL
proof fn test_logical_map_injectivity(s: Seq<int>, map: spec_fn(int) -> int)
    requires
        s.map_values(map).no_duplicates(),
        s.len() >= 2,
{
    map_values_weakens_no_duplicates::<int, int>(s, map);
    // Try to conclude map is injective on all integers (not just s's elements)
    let a: int = 100;
    let b: int = 200;
    assert(map(a) == map(b) ==> a == b);
}

// Test 3: Try to derive that no_duplicates is equivalent to
// mapped no_duplicates (i.e., the biconditional).
// The spec only proves one direction; the converse is false in general.
// SHOULD FAIL
proof fn test_logical_biconditional(s: Seq<int>, map: spec_fn(int) -> int)
    requires s.no_duplicates()
{
    // The spec does NOT let us conclude the other direction
    assert(s.map_values(map).no_duplicates());
}

}
