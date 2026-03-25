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
proof fn phi_2_insert_overwrites_previous_value<B>(map: Map<nat, B>, key: nat, v1: B, v2: B)
    requires
        map == Map::empty().insert(key, v1),
    ensures
        !map.insert(key, v2).values().contains(v1) || v1 == v2,
{
}

}
