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
proof fn phi_3_insert_overwrite_removes_old_value(map: Map<int, int>, key: int, old_val: int, new_val: int)
    requires
        map === Map::empty().insert(key, old_val),
        old_val != new_val,
    ensures
        !map.insert(key, new_val).values().contains(old_val),
{
}

}
