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
proof fn phi_4_duplicate_seq_to_set_equals_singleton(x: int)
    ensures
        seq![x, x].to_set() == set![x],
{
    let s = seq![x, x];
    let ts = s.to_set();
    assert(s[0] == x);
    assert(s[1] == x);
    assert forall |y| ts.contains(y) <==> set![x].contains(y) by {
        if ts.contains(y) {
            assert(exists|i: int| 0 <= i < s.len() && s[i] == y);
        }
    }
    assert_sets_equal!(ts, set![x]);
}

}
