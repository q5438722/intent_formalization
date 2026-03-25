use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

pub open spec fn flatten_set_seq<A>(sets: Seq<Set<A>>) -> Set<A>
{
    sets.fold_left(Set::<A>::empty(), |s1: Set<A>, s2: Set<A>| s1.union(s2))
}

pub proof fn lemma_flatten_set_seq_spec<A>(sets: Seq<Set<A>>)
    ensures
        (forall |x:A| #[trigger] flatten_set_seq(sets).contains(x) ==>
            exists |i: int| 0 <= i < sets.len() && #[trigger] sets[i].contains(x)),
        (forall |x:A, i:int| 0 <= i < sets.len() && #[trigger] sets[i].contains(x) ==>
            flatten_set_seq(sets).contains(x))
    decreases sets.len()
{
    if sets.len() == 0 {
    } else {
        lemma_flatten_set_seq_spec(sets.drop_last());
        assert forall |x:A| flatten_set_seq(sets).contains(x) implies
            exists |i: int| 0 <= i < sets.len() && #[trigger] sets[i].contains(x) by {
            if sets.last().contains(x) {
            } else {
                assert(flatten_set_seq(sets.drop_last()).contains(x));
            }
        }
        assert forall |x:A, i:int| 0 <= i < sets.len() && #[trigger] sets[i].contains(x) implies
            flatten_set_seq(sets).contains(x) by {
            if i == sets.len() - 1 {
                assert(sets.last().contains(x));
                assert(flatten_set_seq(sets) == flatten_set_seq(sets.drop_last()).union(sets.last()));
            } else {
                assert(0 <= i < sets.drop_last().len() && sets.drop_last()[i].contains(x));
            }
        }
    }
}

// Test 1: Uniqueness — claim an element cannot appear in two different sets
// The spec allows overlapping sets, so this is NOT guaranteed
// SHOULD FAIL
proof fn test_logical_uniqueness_of_membership() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(1int);
    let sets = Seq::<Set<int>>::empty().push(s1).push(s2);
    lemma_flatten_set_seq_spec(sets);
    assert(!(sets[0int].contains(1int) && sets[1int].contains(1int)));
}

// Test 2: Order dependence — claim reversing sequence order loses elements
// Set union is commutative, so order should NOT matter
// SHOULD FAIL
proof fn test_logical_order_dependence() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(2int);
    let sets_21 = Seq::<Set<int>>::empty().push(s2).push(s1);
    lemma_flatten_set_seq_spec(sets_21);
    assert(!flatten_set_seq(sets_21).contains(1int));
}

// Test 3: Cross-sequence misuse — calling lemma on one sequence should
// NOT let us conclude membership in a different sequence's flatten
// SHOULD FAIL
proof fn test_logical_cross_sequence_misuse() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(2int);
    let sets = Seq::<Set<int>>::empty().push(s1).push(s2);
    lemma_flatten_set_seq_spec(sets);
    let other_sets = Seq::<Set<int>>::empty().push(s1);
    assert(flatten_set_seq(other_sets).contains(2int));
}

}
