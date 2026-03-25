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

// ===== BOUNDARY TESTS =====

// Test 1: Both sets empty — falsely assert the mapped union is non-empty
// (empty + empty).map(id) == empty. Claiming it contains 0 is false.
// SHOULD FAIL
proof fn test_boundary_both_empty_nonempty()
{
    let s1 = Set::<int>::empty();
    let s2 = Set::<int>::empty();
    let f = |x: int| x;
    set_map_union::<int, int>(s1, s2, f);
    assert((s1 + s2).map(f).contains(0int));
}

// Test 2: One empty, one singleton — falsely assert mapped union is empty
// (empty + {1}).map(id) == {1}, not empty.
// SHOULD FAIL
proof fn test_boundary_one_empty_result_empty()
{
    let s1 = Set::<int>::empty();
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| x;
    set_map_union::<int, int>(s1, s2, f);
    assert((s1 + s2).map(f) =~= Set::<int>::empty());
}

// Test 3: Self-union — falsely claim mapped self-union has unmapped value
// (s + s) == s, so (s + s).map(f) == s.map(f) == {10}. 5 is NOT in result.
// SHOULD FAIL
proof fn test_boundary_self_union_extra_element()
{
    let s = Set::<int>::empty().insert(5int);
    let f = |x: int| (x * 2) as int;
    set_map_union::<int, int>(s, s, f);
    assert((s + s).map(f).contains(5int));
}

// Test 4: Constant function — falsely claim original value persists through map
// ({0} + {1}).map(|x| 42) == {42}. 0 is NOT in result.
// SHOULD FAIL
proof fn test_boundary_constant_fn_original_value_persists()
{
    let s1 = Set::<int>::empty().insert(0int);
    let s2 = Set::<int>::empty().insert(1int);
    let f = |x: int| 42int;
    set_map_union::<int, int>(s1, s2, f);
    assert((s1 + s2).map(f).contains(0int));
}

}
