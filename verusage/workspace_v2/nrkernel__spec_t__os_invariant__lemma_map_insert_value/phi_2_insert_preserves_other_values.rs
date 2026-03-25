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
proof fn phi_2_insert_preserves_other_values(map: Map<int, int>, key1: int, key2: int, val1: int, val2: int)
    requires
        map.contains_key(key2),
        map[key2] == val2,
        key1 != key2,
    ensures
        map.insert(key1, val1).values().contains(val2),
{
}

}
