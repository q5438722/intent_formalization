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
// LOGICAL TEST 1: Different total maps are equal
// Two total maps with genuinely different endpoints should NOT
// be extensionally equal. Tests determinism / uniqueness.
// ============================================================
// SHOULD FAIL
proof fn lt_different_maps_equal<K: KeyTrait + VerusClone>(
    ep1: AbstractEndPoint,
    ep2: AbstractEndPoint,
)
    requires
        ep1.valid_physical_address(),
        ep2.valid_physical_address(),
        ep1 != ep2,
{
    let m1: Map<K, AbstractEndPoint> = Map::total(|k: K| ep1);
    let m2: Map<K, AbstractEndPoint> = Map::total(|k: K| ep2);
    assert(m1 =~= m2);
}

// ============================================================
// LOGICAL TEST 2: Stronger address bound
// valid_physical_address only requires id.len() < 0x100000.
// A much stronger bound (< 0x100) should NOT be derivable.
// ============================================================
// SHOULD FAIL
proof fn lt_stronger_address_bound(ep: AbstractEndPoint)
    requires ep.valid_physical_address(),
{
    assert(ep.id.len() < 0x100);
}

// ============================================================
// LOGICAL TEST 3: Total map has finite domain
// Map::total produces a map whose domain is full (all keys).
// For a generic key type, this domain should NOT be provably
// finite. Tests structural assumption about domain cardinality.
// ============================================================
// SHOULD FAIL
proof fn lt_total_map_finite_domain<K: KeyTrait + VerusClone>() {
    let ep = AbstractEndPoint { id: Seq::new(5 as nat, |i: int| 0u8) };
    let m: Map<K, AbstractEndPoint> = Map::total(|k: K| ep);
    assert(m.dom().finite());
}

}
