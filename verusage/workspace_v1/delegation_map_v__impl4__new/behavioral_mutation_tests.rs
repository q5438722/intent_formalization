use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl AbstractEndPoint {
    pub open spec fn valid_physical_address(self) -> bool {
        self.id.len() < 0x100000
    }
}

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
    spec fn zero_spec() -> Self where Self: std::marker::Sized;
    spec fn cmp_spec(self, other: Self) -> Ordering;
}

pub trait VerusClone : Sized {}

// ============================================================
// BEHAVIORAL MUTATION TEST 1: Wrong mapping value
// Map::total(|k| ep1) maps every key to ep1, not to a
// different endpoint ep2. Asserting the wrong value should fail.
// ============================================================
// SHOULD FAIL
proof fn bmt_wrong_value<K: KeyTrait + VerusClone>(key: K) {
    let ep1 = AbstractEndPoint { id: Seq::new(5 as nat, |i: int| 0u8) };
    let ep2 = AbstractEndPoint { id: Seq::new(5 as nat, |i: int| 1u8) };
    let m: Map<K, AbstractEndPoint> = Map::total(|k: K| ep1);
    assert(m[key] == ep2);
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Non-uniform mapping
// In Map::total(|k| ep), ALL keys map to the same value.
// Asserting two keys map to different values should fail.
// ============================================================
// SHOULD FAIL
proof fn bmt_nonuniform_mapping<K: KeyTrait + VerusClone>(k1: K, k2: K) {
    let ep = AbstractEndPoint { id: Seq::new(5 as nat, |i: int| 0u8) };
    let m: Map<K, AbstractEndPoint> = Map::total(|k: K| ep);
    assert(m[k1] != m[k2]);
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Domain not full
// Map::total always produces a map with a full domain.
// Asserting a specific key is NOT in the domain should fail.
// ============================================================
// SHOULD FAIL
proof fn bmt_domain_missing_key<K: KeyTrait + VerusClone>(key: K) {
    let ep = AbstractEndPoint { id: Seq::new(5 as nat, |i: int| 0u8) };
    let m: Map<K, AbstractEndPoint> = Map::total(|k: K| ep);
    assert(!m.dom().contains(key));
}

}
