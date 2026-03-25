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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// ============================================================

// SHOULD FAIL: Mutation — drop t's contribution, claim union equals just s.to_set()
proof fn test_mutation_1_drop_second_operand()
{
    let s = Seq::<int>::empty().push(1int);
    let t = Seq::<int>::empty().push(2int);
    lemma_to_set_union_auto::<int>();
    // Mutated: should be s.to_set() + t.to_set(), not just s.to_set()
    assert((s + t).to_set() =~= s.to_set());
}

// SHOULD FAIL: Mutation — drop s's contribution, claim union equals just t.to_set()
proof fn test_mutation_2_drop_first_operand()
{
    let s = Seq::<int>::empty().push(1int);
    let t = Seq::<int>::empty().push(2int);
    lemma_to_set_union_auto::<int>();
    // Mutated: should be s.to_set() + t.to_set(), not just t.to_set()
    assert((s + t).to_set() =~= t.to_set());
}

// SHOULD FAIL: Mutation — replace union with intersection (disjoint inputs)
proof fn test_mutation_3_union_to_intersection()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let t = Seq::<int>::empty().push(3int).push(4int);
    lemma_to_set_union_auto::<int>();
    // Mutated: intersection instead of union; disjoint sets => intersection is empty
    assert((s + t).to_set() =~= s.to_set().intersect(t.to_set()));
}

// SHOULD FAIL: Mutation — negate the ensures clause for specific inputs
proof fn test_mutation_4_negate_ensures()
{
    let s = Seq::<int>::empty().push(10int);
    let t = Seq::<int>::empty().push(20int);
    lemma_to_set_union_auto::<int>();
    // Negation of the ensures: claim the sets are NOT equal
    assert((s + t).to_set() !== s.to_set() + t.to_set());
}

// SHOULD FAIL: Mutation — claim result is empty for non-empty inputs
proof fn test_mutation_5_result_is_empty()
{
    let s = Seq::<int>::empty().push(1int);
    let t = Seq::<int>::empty().push(2int);
    lemma_to_set_union_auto::<int>();
    // (s + t).to_set() == {1, 2}, not empty
    assert((s + t).to_set() =~= Set::<int>::empty());
}

}
