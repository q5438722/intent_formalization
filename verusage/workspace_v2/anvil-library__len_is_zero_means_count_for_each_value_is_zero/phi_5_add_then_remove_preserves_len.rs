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
proof fn phi_5_add_then_remove_preserves_len(m: Multiset<int>, v: int)
    ensures
        m.insert(v).remove(v).len() == m.len(),
{
}

}
