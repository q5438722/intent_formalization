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

// Test 1: Negate backward direction — element in sets[0] asserted NOT in flatten
// SHOULD FAIL
proof fn test_mutation_negate_backward() {
    let s = Set::<int>::empty().insert(5int);
    let sets = Seq::<Set<int>>::empty().push(s);
    lemma_flatten_set_seq_spec(sets);
    assert(!flatten_set_seq(sets).contains(5int));
}

// Test 2: Wrong witness index — element 1 is in sets[0], assert it is in sets[1]
// SHOULD FAIL
proof fn test_mutation_wrong_index() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(2int);
    let sets = Seq::<Set<int>>::empty().push(s1).push(s2);
    lemma_flatten_set_seq_spec(sets);
    assert(sets[1int].contains(1int));
}

// Test 3: Phantom element — element not in any set asserted to be in flatten
// SHOULD FAIL
proof fn test_mutation_phantom_element() {
    let s1 = Set::<int>::empty().insert(10int);
    let s2 = Set::<int>::empty().insert(20int);
    let sets = Seq::<Set<int>>::empty().push(s1).push(s2);
    lemma_flatten_set_seq_spec(sets);
    assert(flatten_set_seq(sets).contains(99int));
}

}
