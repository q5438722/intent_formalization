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

// Test 1: Empty sequence — flatten should be empty, asserting it contains an element
// SHOULD FAIL
proof fn test_boundary_empty_seq_contains_element() {
    let sets = Seq::<Set<int>>::empty();
    lemma_flatten_set_seq_spec(sets);
    assert(flatten_set_seq(sets).contains(0int));
}

// Test 2: Sequence of empty sets — flatten should still be empty
// SHOULD FAIL
proof fn test_boundary_all_empty_sets_contains_element() {
    let empty = Set::<int>::empty();
    let sets = Seq::<Set<int>>::empty().push(empty).push(empty);
    lemma_flatten_set_seq_spec(sets);
    assert(flatten_set_seq(sets).contains(42int));
}

// Test 3: Out-of-bounds negative index access on the sequence
// SHOULD FAIL
proof fn test_boundary_negative_index_access() {
    let s = Set::<int>::empty().insert(1int);
    let sets = Seq::<Set<int>>::empty().push(s);
    lemma_flatten_set_seq_spec(sets);
    assert(sets[-1int].contains(1int));
}

}
