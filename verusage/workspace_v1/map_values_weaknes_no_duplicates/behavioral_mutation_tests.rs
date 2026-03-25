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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Assert the converse — s.no_duplicates() does NOT imply
// s.map_values(f).no_duplicates() for a non-injective f.
// s = [1, -1], f = |x| x * x. s has no dups but mapped = [1, 1] has dups.
// SHOULD FAIL
proof fn test_mutation_converse_direction() {
    let s: Seq<int> = Seq::empty().push(1int).push(-1int);
    let f: spec_fn(int) -> int = |x: int| x * x;
    // s has no duplicates (1 != -1), but mapped has duplicates (1 == 1)
    assert(s.no_duplicates());
    assert(s.map_values(f).no_duplicates());
}

// Test 2: Mutate the postcondition to claim the sequence must be empty.
// Valid call with valid inputs, but asserting len() == 0 afterward is wrong.
// SHOULD FAIL
proof fn test_mutation_assert_empty_after_call() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    let f: spec_fn(int) -> int = |x: int| x;
    // Precondition holds: [1, 2] has no duplicates under identity map
    map_values_weakens_no_duplicates::<int, int>(s, f);
    // Postcondition gives s.no_duplicates(), but NOT s.len() == 0
    assert(s.len() == 0);
}

// Test 3: Mutate the postcondition to claim duplicates exist.
// After valid call, assert that the sequence HAS duplicates (negation of actual postcondition).
// SHOULD FAIL
proof fn test_mutation_negate_postcondition() {
    let s: Seq<int> = Seq::empty().push(3int).push(7int);
    let f: spec_fn(int) -> int = |x: int| x + 1;
    // Precondition holds: [4, 8] has no duplicates
    map_values_weakens_no_duplicates::<int, int>(s, f);
    // Try to assert the negation of the postcondition
    assert(!s.no_duplicates());
}

}
