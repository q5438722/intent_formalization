use vstd::prelude::*;

fn main() {}

verus! {

pub enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Ordering {
    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }
}

pub trait KeyTrait : Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;
}

pub trait VerusClone : Sized {}

pub struct EndPoint {
    pub id: Seq<u8>,
}

type ID = EndPoint;

// ============================================================
// BEHAVIORAL MUTATION TEST 1: Empty map contains a key
// Postcondition of new(): s@ == Map::<K,ID>::empty().
// Mutation: claim the empty map contains some key k.
// ============================================================
// SHOULD FAIL
proof fn bmt_empty_map_contains_key<K: KeyTrait + VerusClone>(k: K) {
    let m: Map<K, ID> = Map::empty();
    assert(m.dom().contains(k));
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Empty map has an element
// Postcondition of new(): s@ == Map::<K,ID>::empty().
// Mutation: claim there exists some key in the domain.
// ============================================================
// SHOULD FAIL
proof fn bmt_empty_map_has_element<K: KeyTrait + VerusClone>() {
    let m: Map<K, ID> = Map::empty();
    assert(exists |k: K| m.dom().contains(k));
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Two empty maps are different
// Two calls to new() both produce Map::empty(). Claiming they
// are not extensionally equal is a behavioral mutation.
// ============================================================
// SHOULD FAIL
proof fn bmt_two_empty_maps_differ<K: KeyTrait + VerusClone>() {
    let m1: Map<K, ID> = Map::empty();
    let m2: Map<K, ID> = Map::empty();
    assert(!(m1 =~= m2));
}

}
