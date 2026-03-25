use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemmas under test
#[verifier::external_body]
#[verifier::spinoff_prover]
pub proof fn lemma_to_set_distributes_over_addition<A>(s: Seq<A>, t: Seq<A>)
    ensures (s+t).to_set() == s.to_set() + t.to_set()
{
    unimplemented!()
}

#[verifier::spinoff_prover]
pub proof fn lemma_to_set_union_auto<A>()
    ensures forall |s: Seq<A>, t: Seq<A>| #[trigger] (s+t).to_set() == s.to_set() + t.to_set()
{
    assert forall |s: Seq<A>, t: Seq<A>| #[trigger] (s+t).to_set() == s.to_set() + t.to_set() by {
        lemma_to_set_distributes_over_addition(s, t);
    }
}

// ============================================================
// BOUNDARY TESTS: Edge cases that should be rejected
// ============================================================

// SHOULD FAIL: Concatenation of two empty sequences cannot produce a non-empty set
proof fn test_boundary_1_empty_concat_is_nonempty()
{
    let s = Seq::<int>::empty();
    let t = Seq::<int>::empty();
    lemma_to_set_union_auto::<int>();
    // (empty + empty).to_set() == empty_set + empty_set == empty_set
    // Claiming it's NOT empty should fail
    assert((s + t).to_set() !== Set::<int>::empty());
}

// SHOULD FAIL: Concatenation with empty from right should NOT change the set
proof fn test_boundary_2_concat_empty_right_changes_set()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let t = Seq::<int>::empty();
    lemma_to_set_union_auto::<int>();
    // (s + empty).to_set() == s.to_set() + empty.to_set() == s.to_set()
    // Claiming they differ should fail
    assert((s + t).to_set() !== s.to_set());
}

// SHOULD FAIL: Concatenation with empty from left should NOT change the set
proof fn test_boundary_3_concat_empty_left_changes_set()
{
    let s = Seq::<int>::empty();
    let t = Seq::<int>::empty().push(3int).push(4int);
    lemma_to_set_union_auto::<int>();
    // (empty + t).to_set() == empty.to_set() + t.to_set() == t.to_set()
    // Claiming they differ should fail
    assert((s + t).to_set() !== t.to_set());
}

// SHOULD FAIL: Self-concatenation does NOT make the set empty
proof fn test_boundary_4_self_concat_gives_empty_set()
{
    let s = Seq::<int>::empty().push(5int);
    lemma_to_set_union_auto::<int>();
    // (s + s).to_set() == s.to_set() + s.to_set() == s.to_set() == {5}
    // Claiming it's empty should fail
    assert((s + s).to_set() =~= Set::<int>::empty());
}

// SHOULD FAIL: Concatenated set contains an element that is in neither sequence
proof fn test_boundary_5_phantom_element_in_union()
{
    let s = Seq::<int>::empty().push(1int);
    let t = Seq::<int>::empty().push(2int);
    lemma_to_set_union_auto::<int>();
    // (s + t).to_set() == {1, 2}, should NOT contain 99
    assert((s + t).to_set().contains(99int));
}

}
