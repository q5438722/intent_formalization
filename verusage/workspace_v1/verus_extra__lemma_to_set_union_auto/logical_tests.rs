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
// LOGICAL TESTS: Properties NOT guaranteed by the spec
// ============================================================

// SHOULD FAIL: Stronger property — set cardinality is additive
// False when s and t share elements (|A ∪ B| ≤ |A| + |B|)
proof fn test_logical_1_cardinality_additive()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let t = Seq::<int>::empty().push(2int).push(3int);
    lemma_to_set_union_auto::<int>();
    // s.to_set() = {1,2} (len 2), t.to_set() = {2,3} (len 2)
    // (s+t).to_set() = {1,2,3} (len 3), but 2 + 2 = 4 ≠ 3
    assert((s + t).to_set().len() == s.to_set().len() + t.to_set().len());
}

// SHOULD FAIL: Set difference recovers the other operand
// False when s and t share elements
proof fn test_logical_2_difference_recovers_operand()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let t = Seq::<int>::empty().push(2int).push(3int);
    lemma_to_set_union_auto::<int>();
    // (s+t).to_set() \ s.to_set() = {3}, not t.to_set() = {2,3}
    assert((s + t).to_set().difference(s.to_set()) =~= t.to_set());
}

// SHOULD FAIL: Sequence length equals set cardinality after concatenation
// False when sequences have duplicate elements
proof fn test_logical_3_seq_len_eq_set_len()
{
    let s = Seq::<int>::empty().push(1int).push(1int);
    let t = Seq::<int>::empty().push(2int).push(2int);
    lemma_to_set_union_auto::<int>();
    // (s+t) has len 4, but (s+t).to_set() = {1,2} has len 2
    assert((s + t).to_set().len() == (s + t).len());
}

// SHOULD FAIL: Disjointness of component sets is NOT guaranteed
// The spec says nothing about the intersection of s.to_set() and t.to_set()
proof fn test_logical_4_disjointness_guaranteed()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let t = Seq::<int>::empty().push(2int).push(3int);
    lemma_to_set_union_auto::<int>();
    // s.to_set() ∩ t.to_set() = {2}, NOT empty
    assert(s.to_set().intersect(t.to_set()) =~= Set::<int>::empty());
}

// SHOULD FAIL: Concatenation strictly enlarges the set
// False when all elements of t are already in s
proof fn test_logical_5_concat_strictly_enlarges()
{
    let s = Seq::<int>::empty().push(1int).push(2int).push(3int);
    let t = Seq::<int>::empty().push(1int).push(2int);
    lemma_to_set_union_auto::<int>();
    // (s+t).to_set() = s.to_set() + t.to_set() = {1,2,3} = s.to_set()
    // NOT strictly larger
    assert((s + t).to_set().len() > s.to_set().len());
}

}
