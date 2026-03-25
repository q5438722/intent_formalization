use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: set_lib_ext_v.rs
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




// === Entailment query ===
proof fn phi_3_flatten_order_independent(s1: Set<int>, s2: Set<int>)
    ensures
        flatten_set_seq(seq![s1, s2]) == flatten_set_seq(seq![s2, s1]),
{
    lemma_flatten_set_seq_spec(seq![s1, s2]);
    lemma_flatten_set_seq_spec(seq![s2, s1]);
    assert(flatten_set_seq(seq![s1, s2]) =~= flatten_set_seq(seq![s2, s1]));
}

}
