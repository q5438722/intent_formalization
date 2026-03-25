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

// ===== BOUNDARY TESTS =====

// Test 1: Call map_finite on an arbitrary set without establishing finiteness.
// The precondition requires s.finite(); omitting it should cause a verification failure.
// SHOULD FAIL
proof fn test_boundary_no_finite_precondition()
{
    let s: Set<int> = Set::empty().insert(1int).insert(2int).insert(3int);
    let f = |x: int| (x + 1) as int;
    // Do NOT prove s.finite() — call map_finite directly
    // Without establishing finiteness, precondition is violated
    let t: Set<int> = Set::full();
    map_finite::<int, int>(t, f);
    assert(t.map(f).finite());
}

// Test 2: Use the full (infinite) set and try to conclude its map is finite.
// Set::full() is not finite, so the implication s.finite() ==> ... does not fire.
// SHOULD FAIL
proof fn test_boundary_infinite_set_map_finite()
{
    map_set_finite_auto::<int, int>();
    let s: Set<int> = Set::full();
    let f = |x: int| x;
    // The universal quantifier only applies when s.finite().
    // s is infinite, so we cannot conclude s.map(f).finite().
    assert(s.map(f).finite());
}

// Test 3: Complement of a singleton is infinite — assert its map is finite.
// Set::full().remove(0) is still infinite.
// SHOULD FAIL
proof fn test_boundary_complement_singleton_finite()
{
    map_set_finite_auto::<int, int>();
    let s: Set<int> = Set::full().remove(0int);
    let f = |x: int| x;
    // s is infinite (all ints except 0), so map(f) should not be provably finite
    assert(s.map(f).finite());
}

// Test 4: Constant function on infinite set — map image is {c}, but precondition not met.
// Even though the image would semantically be finite, the spec requires s.finite().
// SHOULD FAIL
proof fn test_boundary_constant_fn_infinite_set()
{
    let s: Set<int> = Set::full();
    let f = |x: int| 42int;
    map_finite::<int, int>(s, f);
    assert(s.map(f).finite());
}

}
