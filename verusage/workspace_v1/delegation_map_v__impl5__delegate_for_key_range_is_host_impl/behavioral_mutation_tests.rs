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

// ==================== BEHAVIORAL MUTATION TESTS ====================

// Test 1: If all keys in range [5, 10) map to id_correct, claiming they
// map to a DIFFERENT id_wrong should be rejected.
// SHOULD FAIL
proof fn mutation_wrong_host(m: Map<AbstractKey, AbstractEndPoint>, id_correct: AbstractEndPoint, id_wrong: AbstractEndPoint)
    requires
        m.dom().is_full(),
        id_correct != id_wrong,
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let kr = KeyRange { lo, hi };
    let dm = AbstractDelegationMap(m);

    assume(dm.delegate_for_key_range_is_host(kr, id_correct));
    // Mutated output: wrong host
    assert(dm.delegate_for_key_range_is_host(kr, id_wrong));
}

// Test 2: delegate_for_key_range_is_host for [5, 10) does NOT imply
// a key OUTSIDE the range (ukey=15) maps to the same host.
// SHOULD FAIL
proof fn mutation_key_outside_range(m: Map<AbstractKey, AbstractEndPoint>)
    requires
        m.dom().is_full(),
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let kr = KeyRange { lo, hi };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);
    let k_outside = SHTKey { ukey: 15 };

    assume(dm.delegate_for_key_range_is_host(kr, id));
    // Key 15 is outside [5, 10) — spec gives no guarantee
    assert(dm[k_outside] == id);
}

// Test 3: delegate_for_key_range_is_host for [5, 10) does NOT imply
// the same property for the EXTENDED range [5, 11).
// SHOULD FAIL
proof fn mutation_extend_range(m: Map<AbstractKey, AbstractEndPoint>)
    requires
        m.dom().is_full(),
{
    let lo = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 5 });
    let hi = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 10 });
    let hi_extended = KeyIterator::<AbstractKey>::new_spec(SHTKey { ukey: 11 });
    let kr = KeyRange { lo, hi };
    let kr_extended = KeyRange { lo, hi: hi_extended };
    let id = AbstractEndPoint { id: Seq::empty() };
    let dm = AbstractDelegationMap(m);

    assume(dm.delegate_for_key_range_is_host(kr, id));
    // Extended range includes key 10, which is NOT guaranteed
    assert(dm.delegate_for_key_range_is_host(kr_extended, id));
}

} // verus!
