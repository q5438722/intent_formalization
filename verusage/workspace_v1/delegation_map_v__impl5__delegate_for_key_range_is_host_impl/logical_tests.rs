use vstd::prelude::*;

fn main() {}

verus! {

// === Type Definitions ===

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl AbstractEndPoint {
    pub open spec fn valid_physical_address(self) -> bool {
        self.id.len() < 0x100000
    }
}

pub struct AbstractDelegationMap(pub Map<AbstractKey, AbstractEndPoint>);

impl AbstractDelegationMap {
    pub open spec fn view(self) -> Map<AbstractKey, AbstractEndPoint> {
        self.0
    }

    pub open spec fn spec_index(self, key: AbstractKey) -> AbstractEndPoint
        recommends self.0.dom().contains(key)
    {
        self@.index(key)
    }

    pub open spec fn is_complete(self) -> bool {
        self@.dom().is_full()
    }

    pub open spec fn delegate_for_key_range_is_host(self, kr: KeyRange<AbstractKey>, id: AbstractEndPoint) -> bool
        recommends self.is_complete(),
    {
        forall |k: AbstractKey| #[trigger] kr.contains(k) ==> self[k] == id
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

pub struct SHTKey {
    pub ukey: u64,
}

impl KeyTrait for SHTKey {
    open spec fn zero_spec() -> Self { SHTKey { ukey: 0 } }
    open spec fn cmp_spec(self, other: Self) -> Ordering {
        if self.ukey < other.ukey { Ordering::Less }
        else if self.ukey == other.ukey { Ordering::Equal }
        else { Ordering::Greater }
    }
}

impl VerusClone for SHTKey {}

pub type AbstractKey = SHTKey;

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
        || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }
}

pub struct KeyRange<K: KeyTrait + VerusClone> {
    pub lo: KeyIterator<K>,
    pub hi: KeyIterator<K>,
}

impl<K: KeyTrait + VerusClone> KeyRange<K> {
    pub open spec fn contains(self, k: K) -> bool {
        KeyIterator::<K>::between(self.lo, KeyIterator::<K>::new_spec(k), self.hi)
    }
}

// ==================== LOGICAL TESTS ====================

// Test 1: A complete map does NOT imply all keys map to the same endpoint.
// The spec allows different keys to map to different hosts.
// SHOULD FAIL
proof fn logical_global_uniformity(m: Map<AbstractKey, AbstractEndPoint>)
    requires
        m.dom().is_full(),
{
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);
    // Completeness alone says nothing about uniform delegation
    assert(forall |k: AbstractKey| dm[k] == id);
}

// Test 2: delegate_for_key_range_is_host for [5, 10) does NOT imply
// the same for [2, 10) — left-extending the range is not entailed.
// SHOULD FAIL
proof fn logical_range_extension_left(m: Map<AbstractKey, AbstractEndPoint>)
    requires
        m.dom().is_full(),
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let lo_earlier = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 2 });
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let kr = KeyRange { lo, hi };
    let kr_extended = KeyRange { lo: lo_earlier, hi };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);

    assume(dm.delegate_for_key_range_is_host(kr, id));
    // Cannot extend range leftward — keys in [2, 5) are unconstrained
    assert(dm.delegate_for_key_range_is_host(kr_extended, id));
}

// Test 3: Two disjoint ranges [0,5) and [5,10) mapping to different hosts
// is a perfectly valid configuration — should NOT derive false.
// SHOULD FAIL
proof fn logical_disjoint_ranges_different_hosts_not_contradictory(
    m: Map<AbstractKey, AbstractEndPoint>,
    id1: AbstractEndPoint,
    id2: AbstractEndPoint,
)
    requires
        m.dom().is_full(),
        id1 != id2,
{
    let lo1 = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 0 });
    let hi1 = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let lo2 = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let hi2 = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let kr1 = KeyRange { lo: lo1, hi: hi1 };
    let kr2 = KeyRange { lo: lo2, hi: hi2 };
    let dm = AbstractDelegationMap(m);

    assume(dm.delegate_for_key_range_is_host(kr1, id1));
    assume(dm.delegate_for_key_range_is_host(kr2, id2));
    // This is a valid configuration; cannot derive contradiction
    assert(false);
}

} // verus!
