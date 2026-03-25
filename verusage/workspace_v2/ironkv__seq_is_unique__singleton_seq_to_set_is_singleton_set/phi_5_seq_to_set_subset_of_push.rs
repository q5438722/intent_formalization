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
proof fn phi_5_seq_to_set_subset_of_push(s: Seq<int>, x: int)
    ensures
        s.to_set().subset_of(s.push(x).to_set()),
{
    let s2 = s.push(x);
    assert forall |y| s.to_set().contains(y) implies s2.to_set().contains(y) by {
        let i = choose|i: int| 0 <= i < s.len() && s[i] == y;
        assert(s2[i] == y);
    }
}

}
