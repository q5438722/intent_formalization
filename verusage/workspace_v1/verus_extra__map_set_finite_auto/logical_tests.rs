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

// ===== LOGICAL TESTS =====

// Test 1: Converse — if s.map(f).finite() then s.finite().
// This is NOT entailed. A constant function on an infinite set produces a finite image.
// Example: f(x) = 0 for all x. Set::full().map(f) = {0} which is finite, but Set::full() is not.
// SHOULD FAIL
proof fn test_logical_converse_finite()
{
    map_set_finite_auto::<int, int>();
    // Try to prove the converse as a universal statement
    // For any s and f, if s.map(f).finite() then s.finite()
    // Use a concrete counterexample scenario
    let s: Set<int> = Set::full();
    let f = |x: int| 0int;
    // Assume s.map(f).finite() (which is semantically true: image = {0})
    // Try to conclude s.finite() — this should fail since s is infinite
    assume(s.map(f).finite());
    assert(s.finite());
}

// Test 2: Cardinality preservation — s.map(f).len() == s.len().
// Not guaranteed for non-injective functions. f(x) = 0: s = {1, 2} maps to {0}.
// |{0}| = 1 ≠ 2 = |{1, 2}|.
// SHOULD FAIL
proof fn test_logical_cardinality_preserved()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| 0int;
    map_finite::<int, int>(s, f);
    // The spec only guarantees finiteness, not cardinality preservation
    assert(s.map(f).len() == s.len());
}

// Test 3: Injectivity of map — s1.map(f) == s2.map(f) implies s1 == s2.
// Not true for non-injective f. s1 = {1}, s2 = {2}, f(x) = 0.
// s1.map(f) = {0} = s2.map(f), but s1 ≠ s2.
// SHOULD FAIL
proof fn test_logical_map_injective()
{
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(2int);
    let f = |x: int| 0int;
    map_finite::<int, int>(s1, f);
    map_finite::<int, int>(s2, f);
    // Both map to {0}, falsely conclude the sets are equal
    assume(s1.map(f) =~= s2.map(f));
    assert(s1 =~= s2);
}

// Test 4: Finiteness transfers across arbitrary functions — if s.map(f).finite()
// for one f, then s.map(g).finite() for any g.
// Not guaranteed. If s is infinite, a constant f gives finite image but
// injective g gives infinite image. The spec does not support this transfer.
// SHOULD FAIL
proof fn test_logical_finiteness_transfers_across_functions()
{
    let s: Set<int> = Set::full();
    let f = |x: int| 0int;
    let g = |x: int| x;
    // s.map(f) = {0} is finite, but s.map(g) = Set::full() is not
    assume(s.map(f).finite());
    // Falsely conclude s.map(g) is also finite
    assert(s.map(g).finite());
}

}
