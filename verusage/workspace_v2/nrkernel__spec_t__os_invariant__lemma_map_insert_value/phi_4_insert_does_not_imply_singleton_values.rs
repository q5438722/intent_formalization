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
proof fn phi_4_insert_does_not_imply_singleton_values(key: nat, value: nat)
    ensures
        Map::empty().insert(key, value).values() =~= set![value],
{
}

}
