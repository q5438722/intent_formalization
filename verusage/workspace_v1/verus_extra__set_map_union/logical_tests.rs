use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemmas (copied from source) =====
#[verifier::external_body]
pub proof fn set_map_union<A, B>(s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B)
    ensures (s1 + s2).map(f) == s1.map(f) + s2.map(f)
{
    unimplemented!()
}

pub proof fn set_map_union_auto<A, B>()
    ensures forall |s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
        #[trigger] (s1 + s2).map(f) == s1.map(f) + s2.map(f)
{
    assert forall |s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
        #[trigger] ((s1 + s2).map(f)) == s1.map(f) + s2.map(f) by {
        set_map_union(s1, s2, f);
    }
}

// ===== LOGICAL TESTS =====

// Test 1: Inconsistency check — try to derive `false` from the spec
// If the external_body axiom introduces inconsistency, this would pass.
// SHOULD FAIL
proof fn test_logical_inconsistency_check()
{
    set_map_union_auto::<int, int>();
    assert(false);
}

// Test 2: Map does NOT distribute over set difference
// The spec only covers union. f(A \ B) ≠ f(A) \ f(B) in general.
// Counter: A={1,2}, B={1}, f=const 0. f(A\B)=f({2})={0}, f(A)\f(B)={0}\{0}={}
// SHOULD FAIL
proof fn test_logical_map_over_difference()
{
    set_map_union_auto::<int, int>();
    let s1 = Set::<int>::empty().insert(1int).insert(2int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| 0int;
    assert(s1.difference(s2).map(f) =~= s1.map(f).difference(s2.map(f)));
}

// Test 3: Map does NOT distribute over intersection for non-injective f
// f(A ∩ B) ⊆ f(A) ∩ f(B) but equality fails without injectivity.
// Counter: A={0,1}, B={2,3}, f=|x|x%2. A∩B={}, f({})={}, f(A)∩f(B)={0,1}∩{0,1}={0,1}
// SHOULD FAIL
proof fn test_logical_map_over_intersection()
{
    set_map_union_auto::<int, int>();
    let s1 = Set::<int>::empty().insert(0int).insert(1int);
    let s2 = Set::<int>::empty().insert(2int).insert(3int);
    let f = |x: int| (x % 2) as int;
    assert(s1.intersect(s2).map(f) =~= s1.map(f).intersect(s2.map(f)));
}

// Test 4: Claim the spec implies map injectivity (preserves distinctness)
// With constant f, mapping a multi-element set collapses to one element.
// Claiming an unmapped value is in the result is false.
// SHOULD FAIL
proof fn test_logical_map_implies_injectivity()
{
    set_map_union_auto::<int, int>();
    let s = Set::<int>::empty().insert(0int).insert(1int);
    let f = |x: int| 42int;
    // s.map(f) = {42}. Falsely claim 0 is in the mapped set.
    assert(s.map(f).contains(0int));
}

}
