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
proof fn phi_4_choose_returns_contained_element(m: Multiset<int>)
    requires
        m.len() > 0,
    ensures
        m.count(m.choose()) > 0,
{
    len_is_zero_means_count_for_each_value_is_zero(m);
}

}
