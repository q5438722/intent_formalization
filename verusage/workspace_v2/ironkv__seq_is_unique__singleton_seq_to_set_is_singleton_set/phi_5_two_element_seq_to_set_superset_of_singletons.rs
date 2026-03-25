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
proof fn phi_5_two_element_seq_to_set_superset_of_singletons(x: int, y: int)
    requires
        x != y,
    ensures
        seq![x, y].to_set() == set![x, y],
{
    let s = seq![x, y];
    assert forall |z| s.to_set().contains(z) <==> set![x, y].contains(z) by {
        if z == x { assert(s[0] == z); }
        if z == y { assert(s[1] == z); }
    }
    assert_sets_equal!(s.to_set(), set![x, y]);
}

}
