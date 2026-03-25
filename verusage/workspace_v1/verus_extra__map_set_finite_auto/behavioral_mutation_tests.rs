use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemmas (copied from source) =====

#[verifier::external_body]
#[verifier::spinoff_prover]
pub proof fn map_finite<A, B>(s: Set<A>, f: spec_fn(A) -> B)
requires
    s.finite(),
ensures
    s.map(f).finite(),
{
    unimplemented!()
}

#[verifier::spinoff_prover]
pub proof fn map_set_finite_auto<A, B>()
ensures
    forall |s: Set<A>, f: spec_fn(A) -> B| s.finite() ==> #[trigger] (s.map(f).finite()),
{
    assert forall |s: Set<A>, f: spec_fn(A) -> B| s.finite() implies #[trigger] s.map(f).finite() by {
        map_finite(s, f);
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Negate postcondition — assert that mapping a finite set yields an infinite result.
// The spec guarantees s.map(f).finite(); negating this should fail.
// SHOULD FAIL
proof fn test_mutation_negate_postcondition()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| (x * 2) as int;
    map_finite::<int, int>(s, f);
    // After calling map_finite, we know s.map(f).finite()
    // Falsely assert the opposite
    assert(!s.map(f).finite());
}

// Test 2: Assert mapped set equals the empty set for a non-empty finite set.
// s = {1, 2}, f = id => s.map(f) = {1, 2} ≠ {}.
// SHOULD FAIL
proof fn test_mutation_map_result_is_empty()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| x;
    map_finite::<int, int>(s, f);
    // Falsely assert the mapped set is empty
    assert(s.map(f) =~= Set::<int>::empty());
}

// Test 3: Assert mapped set equals the original set under a non-identity function.
// s = {1, 2}, f(x) = x + 10 => s.map(f) = {11, 12} ≠ {1, 2}.
// SHOULD FAIL
proof fn test_mutation_map_equals_original()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| (x + 10) as int;
    map_finite::<int, int>(s, f);
    // Falsely assert s.map(f) == s
    assert(s.map(f) =~= s);
}

// Test 4: Swap function — establish finiteness with f but assert result about g.
// s = {0, 1}, f(x) = x, g(x) = x + 100.
// map_finite gives s.map(f).finite(). g is a different function.
// Falsely assert s.map(g) equals s.map(f).
// s.map(f) = {0, 1}, s.map(g) = {100, 101}.
// SHOULD FAIL
proof fn test_mutation_swap_function()
{
    let s = Set::<int>::empty().insert(0int).insert(1int);
    let f = |x: int| x;
    let g = |x: int| (x + 100) as int;
    map_finite::<int, int>(s, f);
    // Falsely assert s.map(g) == s.map(f)
    assert(s.map(g) =~= s.map(f));
}

}
