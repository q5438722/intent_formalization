use vstd::prelude::*;
use vstd::set_lib::*;

fn main() {}

verus! {

    pub proof fn singleton_seq_to_set_is_singleton_set<T>(x: T)
        ensures seq![x].to_set() == set![x]
    {
        let seq1 = seq![x];
        let set1 = seq1.to_set();
        let set2 = set![x];
        assert forall |y| set1.contains(y) <==> set2.contains(y) by
        {
            if y == x {
                assert (seq1[0] == y);
                assert (set1.contains(y));
            }
        }
        assert_sets_equal!(seq![x].to_set(), set![x]);
    }



// === Entailment query ===
proof fn phi_1_singleton_seq_to_set_len<T>(x: T)
    ensures
        seq![x].to_set().len() == 1,
{
    singleton_seq_to_set_is_singleton_set(x);
    assert(seq![x].to_set() == set![x]);
    assert(set![x].len() == 1);
}

}
