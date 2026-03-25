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
proof fn phi_1_insert_value_in_values_trivial(map: Map<int, int>, key: int, value: int)
    ensures
        map.insert(key, value).values().contains(value),
{
    lemma_map_insert_value(map, key, value);
}

}
