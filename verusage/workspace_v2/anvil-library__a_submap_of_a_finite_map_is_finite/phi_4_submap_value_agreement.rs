use vstd::prelude::*;

fn main() {}
verus! {

pub proof fn a_submap_of_a_finite_map_is_finite<K, V>(m1: Map<K, V>, m2: Map<K, V>)
    requires
        m1.submap_of(m2),
        m2.dom().finite(),
    ensures
        m1.dom().finite(),
{
    assert(m1.dom() === m2.dom().intersect(m1.dom()));
}



// === Entailment query ===
proof fn phi_4_submap_value_agreement(m1: Map<int, int>, m2: Map<int, int>, k: int)
    requires
        m1.submap_of(m2),
        m1.contains_key(k),
    ensures
        m2[k] == m1[k],
{
}

}
