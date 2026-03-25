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
// BOUNDARY TEST 1: valid_physical_address at exact boundary
// id.len() == 0x100000 does NOT satisfy < 0x100000.
// ============================================================
// SHOULD FAIL
proof fn bt_address_at_exact_boundary() {
    let ep = AbstractEndPoint { id: Seq::new(0x100000 as nat, |i: int| 0u8) };
    assert(ep.valid_physical_address());
}

// ============================================================
// BOUNDARY TEST 2: valid_physical_address well above boundary
// id.len() == 0x200000 clearly does NOT satisfy < 0x100000.
// ============================================================
// SHOULD FAIL
proof fn bt_address_above_boundary() {
    let ep = AbstractEndPoint { id: Seq::new(0x200000 as nat, |i: int| 0u8) };
    assert(ep.valid_physical_address());
}

// ============================================================
// BOUNDARY TEST 3: Total map with boundary-violating endpoint
// If the endpoint violates valid_physical_address, looking up
// any key in Map::total should NOT yield a valid address.
// This tests whether the precondition of `new` is necessary.
// ============================================================
// SHOULD FAIL
proof fn bt_total_map_invalid_endpoint<K: KeyTrait + VerusClone>(key: K) {
    let ep = AbstractEndPoint { id: Seq::new(0x100000 as nat, |i: int| 0u8) };
    let m: Map<K, AbstractEndPoint> = Map::total(|k: K| ep);
    assert(m[key].valid_physical_address());
}

}
