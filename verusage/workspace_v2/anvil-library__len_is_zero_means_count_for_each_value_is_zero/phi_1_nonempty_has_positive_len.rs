use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

pub proof fn len_is_zero_means_count_for_each_value_is_zero<V>(m: Multiset<V>)
    ensures (forall |v| m.count(v) == 0) == (m.len() == 0),
{
    if m.len() != 0 {
        assert(m.count(m.choose()) > 0);
    }
}



// === Entailment query ===
proof fn phi_1_nonempty_has_positive_len(m: Multiset<int>, v: int)
    requires
        m.count(v) > 0,
    ensures
        m.len() > 0,
{
    len_is_zero_means_count_for_each_value_is_zero(m);
}

}
