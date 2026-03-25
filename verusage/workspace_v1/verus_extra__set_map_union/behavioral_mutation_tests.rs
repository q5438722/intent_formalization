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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Drop s2 contribution — mutated postcondition
// True: (s1 + s2).map(f) == s1.map(f) + s2.map(f)
// Mutated: (s1 + s2).map(f) == s1.map(f)   [drop s2.map(f)]
// SHOULD FAIL
proof fn test_mutation_drop_s2()
{
    let s1 = Set::<int>::empty().insert(0int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| x;
    set_map_union::<int, int>(s1, s2, f);
    // (s1 + s2).map(f) == {0, 1}, but s1.map(f) == {0}
    assert((s1 + s2).map(f) =~= s1.map(f));
}

// Test 2: Drop s1 contribution — mutated postcondition
// True: (s1 + s2).map(f) == s1.map(f) + s2.map(f)
// Mutated: (s1 + s2).map(f) == s2.map(f)   [drop s1.map(f)]
// SHOULD FAIL
proof fn test_mutation_drop_s1()
{
    let s1 = Set::<int>::empty().insert(0int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| x;
    set_map_union::<int, int>(s1, s2, f);
    // (s1 + s2).map(f) == {0, 1}, but s2.map(f) == {1}
    assert((s1 + s2).map(f) =~= s2.map(f));
}

// Test 3: Negate the postcondition — assert the two sides are NOT equal
// The lemma guarantees equality; negation should fail.
// SHOULD FAIL
proof fn test_mutation_negated_postcondition()
{
    let s1 = Set::<int>::empty().insert(0int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| x;
    set_map_union::<int, int>(s1, s2, f);
    assert((s1 + s2).map(f) !== s1.map(f) + s2.map(f));
}

// Test 4: Use a different function on the RHS — wrong function applied
// True: (s1 + s2).map(f) == s1.map(f) + s2.map(f)
// Mutated: (s1 + s2).map(f) == s1.map(g) + s2.map(g) where g ≠ f
// SHOULD FAIL
proof fn test_mutation_wrong_function_rhs()
{
    let s1 = Set::<int>::empty().insert(0int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| (x + 1) as int;
    let g = |x: int| (x * 10) as int;
    set_map_union::<int, int>(s1, s2, f);
    // (s1 + s2).map(f) == {1, 2}
    // s1.map(g) + s2.map(g) == {0} + {10} == {0, 10}
    assert((s1 + s2).map(f) =~= s1.map(g) + s2.map(g));
}

}
