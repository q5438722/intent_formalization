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

// ==================== BOUNDARY TESTS ====================

// Test 1: Empty range (lo == hi at ukey=5) is vacuously satisfied.
// Asserting negation SHOULD FAIL because delegate_for_key_range_is_host
// is trivially true when no key satisfies the range predicate.
// SHOULD FAIL
proof fn boundary_empty_range_is_vacuously_true(m: Map<AbstractKey, AbstractEndPoint>)
    requires m.dom().is_full()
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let hi = lo;
    let kr = KeyRange { lo, hi };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);
    assert(!dm.delegate_for_key_range_is_host(kr, id));
}

// Test 2: Reversed range (hi=3 < lo=10) is also vacuously satisfied.
// No key k can satisfy 10 <= k < 3, so delegate is trivially true.
// SHOULD FAIL
proof fn boundary_reversed_range_is_vacuously_true(m: Map<AbstractKey, AbstractEndPoint>)
    requires m.dom().is_full()
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 3 });
    let kr = KeyRange { lo, hi };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);
    assert(!dm.delegate_for_key_range_is_host(kr, id));
}

// Test 3: Endpoint id length exactly at boundary (0x100000) should NOT
// satisfy valid_physical_address, since the spec requires strictly less.
// SHOULD FAIL
proof fn boundary_address_at_limit() {
    let id_seq: Seq<u8>;
    assume(id_seq.len() == 0x100000);
    let ep = AbstractEndPoint { id: id_seq };
    assert(ep.valid_physical_address());
}

// Test 4: lo = None (end iterator) makes the range empty — no key is >= "end".
// Delegate is vacuously true; asserting negation SHOULD FAIL.
// SHOULD FAIL
proof fn boundary_none_lo_empty_range(m: Map<AbstractKey, AbstractEndPoint>)
    requires m.dom().is_full()
{
    let lo: KeyIterator<AbstractKey> = KeyIterator { k: Option::None };
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 100 });
    let kr = KeyRange { lo, hi };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);
    assert(!dm.delegate_for_key_range_is_host(kr, id));
}

} // verus!
