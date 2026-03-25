use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/os_invariant.rs
pub proof fn lemma_map_insert_value<A, B>(map: Map<A, B>, key: A, value: B)
    requires
    ensures
        map.insert(key, value).values().contains(value),
{
    assert(map.insert(key, value).dom().contains(key));
    assert(map.insert(key, value)[key] == value);
}




// === Entailment query ===
proof fn phi_5_values_contains_not_injective(k1: nat, k2: nat, v: nat)
    requires
        k1 != k2,
    ensures
        Map::empty().insert(k1, v).insert(k2, v).values().contains(v),
{
    lemma_map_insert_value(Map::empty().insert(k1, v), k2, v);
}

}
