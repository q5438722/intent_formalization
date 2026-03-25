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

// ===== BOUNDARY TESTS =====

// Test 1: Empty set — assert that inserting and mapping produces the empty set
// The truth: {}.insert(0).map(id) == {0}, NOT empty.
// SHOULD FAIL
proof fn test_boundary_empty_set_mapped_is_empty()
{
    let s = Set::<int>::empty();
    let f = |x: int| x;
    lemma_set_map_insert::<int, int>(s, f, 0int);
    // s.insert(0).map(id) == s.map(id).insert(0) == {0}
    // Falsely assert the mapped result is empty
    assert(s.insert(0int).map(f) =~= Set::<int>::empty());
}

// Test 2: Empty set — assert mapped result does NOT contain the inserted-then-mapped element
// The truth: {}.insert(0).map(|x| x+1) == {1}, so it DOES contain 1.
// SHOULD FAIL
proof fn test_boundary_empty_set_missing_mapped_element()
{
    let s = Set::<int>::empty();
    let f = |x: int| (x + 1) as int;
    lemma_set_map_insert::<int, int>(s, f, 0int);
    // s.insert(0).map(f) == {1}
    // Falsely assert 1 is not in the result
    assert(!s.insert(0int).map(f).contains(1int));
}

// Test 3: Constant function — assert the mapped result contains the original (unmapped) value
// With f = |x| 42, mapping {5} gives {42}, not {5}.
// SHOULD FAIL
proof fn test_boundary_constant_fn_original_value_in_result()
{
    let s = Set::<int>::empty();
    let f = |x: int| 42int;
    lemma_set_map_insert::<int, int>(s, f, 5int);
    // s.insert(5).map(f) == {42}
    // Falsely assert that 5 is in the mapped set
    assert(s.insert(5int).map(f).contains(5int));
}

// Test 4: Redundant insert (element already present) — falsely claim the mapped set changes
// When x is already in s, s.insert(x) == s, so mapped sets are identical.
// SHOULD FAIL
proof fn test_boundary_redundant_insert_false_extra_element()
{
    let s = Set::<int>::empty().insert(3int);
    let f = |x: int| (x * 2) as int;
    lemma_set_map_insert::<int, int>(s, f, 3int);
    // s = {3}, s.insert(3) = {3}, s.insert(3).map(f) = {6}
    // Falsely claim 3 is in the mapped result
    assert(s.insert(3int).map(f).contains(3int));
}

}
