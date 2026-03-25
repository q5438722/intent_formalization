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

// ===== BOUNDARY TESTS =====

// Test 1: Constant map on a 2-element sequence.
// s.map_values(|x| 0) = [0, 0] has duplicates, violating requires.
// SHOULD FAIL
proof fn test_boundary_constant_map() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| 0int);
}

// Test 2: Identity map on a sequence with duplicate elements.
// s = [1, 1], map = id. Mapped = [1, 1] has duplicates.
// SHOULD FAIL
proof fn test_boundary_duplicate_elements_identity_map() {
    let s: Seq<int> = Seq::empty().push(1int).push(1int);
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| x);
}

// Test 3: Non-injective map on 3-element sequence.
// s = [0, 1, 2], map = |x| x % 2. Mapped = [0, 1, 0] has duplicates.
// SHOULD FAIL
proof fn test_boundary_non_injective_map() {
    let s: Seq<int> = Seq::empty().push(0int).push(1int).push(2int);
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| x % 2);
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 4: Assert the converse — s.no_duplicates() does NOT imply
// s.map_values(f).no_duplicates() for a non-injective f.
// SHOULD FAIL
proof fn test_mutation_converse_direction() {
    let s: Seq<int> = Seq::empty().push(1int).push(-1int);
    let f: spec_fn(int) -> int = |x: int| x * x;
    assert(s.no_duplicates());
    assert(s.map_values(f).no_duplicates());
}

// Test 5: After valid call, assert the sequence must be empty.
// The postcondition guarantees no_duplicates, NOT len() == 0.
// SHOULD FAIL
proof fn test_mutation_assert_empty_after_call() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    let f: spec_fn(int) -> int = |x: int| x;
    map_values_weakens_no_duplicates::<int, int>(s, f);
    assert(s.len() == 0);
}

// Test 6: After valid call, assert the negation of the postcondition.
// SHOULD FAIL
proof fn test_mutation_negate_postcondition() {
    let s: Seq<int> = Seq::empty().push(3int).push(7int);
    let f: spec_fn(int) -> int = |x: int| x + 1;
    map_values_weakens_no_duplicates::<int, int>(s, f);
    assert(!s.no_duplicates());
}

// ===== LOGICAL TESTS =====

// Test 7: Try to derive no_duplicates for an arbitrary sequence
// without any precondition about the mapped sequence.
// SHOULD FAIL
proof fn test_logical_no_dup_without_precondition(s: Seq<int>) {
    assert(s.no_duplicates());
}

// Test 8: Try to derive that the map function must be injective
// on all integers. The spec does not guarantee this.
// SHOULD FAIL
proof fn test_logical_map_injectivity(s: Seq<int>, map: spec_fn(int) -> int)
    requires
        s.map_values(map).no_duplicates(),
        s.len() >= 2,
{
    map_values_weakens_no_duplicates::<int, int>(s, map);
    let a: int = 100;
    let b: int = 200;
    assert(map(a) == map(b) ==> a == b);
}

// Test 9: Try to derive the biconditional (converse direction).
// The spec only proves mapped_no_dup => original_no_dup, not the reverse.
// SHOULD FAIL
proof fn test_logical_biconditional(s: Seq<int>, map: spec_fn(int) -> int)
    requires s.no_duplicates()
{
    assert(s.map_values(map).no_duplicates());
}

}
