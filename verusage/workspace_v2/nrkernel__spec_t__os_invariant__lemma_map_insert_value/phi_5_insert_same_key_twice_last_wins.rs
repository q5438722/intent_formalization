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
proof fn phi_5_insert_same_key_twice_last_wins(map: Map<int, int>, key: int, val1: int, val2: int)
    ensures
        map.insert(key, val1).insert(key, val2)[key] == val2,
{
}

}
