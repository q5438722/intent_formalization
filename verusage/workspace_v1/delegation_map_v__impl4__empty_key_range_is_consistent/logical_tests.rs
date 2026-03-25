use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl Ordering {
    pub open spec fn eq(self) -> bool {
        matches!(self, Ordering::Equal)
    }

    pub open spec fn ne(self) -> bool {
        !matches!(self, Ordering::Equal)
    }

    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }

    pub open spec fn gt(self) -> bool {
        matches!(self, Ordering::Greater)
    }

    pub open spec fn le(self) -> bool {
        !matches!(self, Ordering::Greater)
    }
}

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }
}
#[verifier::reject_recursive_types(K)]

struct StrictlyOrderedMap<K: KeyTrait + VerusClone> {
    keys: StrictlyOrderedVec<K>,
    vals: Vec<ID>,
    m: Ghost<Map<K, ID>>,
}
#[verifier::reject_recursive_types(K)]

pub struct DelegationMap<K: KeyTrait + VerusClone> {
    lows: StrictlyOrderedMap<K>,
    m: Ghost<Map<K, AbstractEndPoint>>,
}

impl<K: KeyTrait + VerusClone> DelegationMap<K> {

    pub closed spec fn view(self) -> Map<K, AbstractEndPoint> {
        self.m@
    }

    pub open spec fn range_consistent(self, lo: &KeyIterator<K>, hi: &KeyIterator<K>, dst: &ID) -> bool {
        forall |k| KeyIterator::between(*lo, KeyIterator::new_spec(k), *hi) ==> (#[trigger] self@[k]) == dst@
    }

    proof fn empty_key_range_is_consistent(&self, lo: &KeyIterator<K>, hi: &KeyIterator<K>, id: &ID)
        requires
            lo.geq_spec(*hi),
        ensures
            self.range_consistent(lo, hi, id),
    {
        K::cmp_properties();
    }
}

pub struct EndPoint {
    pub id: Vec<u8>,
}

impl EndPoint {
    pub open spec fn view(self) -> AbstractEndPoint {
        AbstractEndPoint { id: self.id@ }
    }
}

pub trait KeyTrait: Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;

    proof fn cmp_properties()
        ensures
            forall|a: Self, b: Self| #![auto] a == b <==> a.cmp_spec(b).eq(),
            forall|a: Self| #![auto] a.cmp_spec(a).eq(),
            forall|a: Self, b: Self|
                (#[trigger] a.cmp_spec(b)).eq() == b.cmp_spec(a).eq(),
            forall|a: Self, b: Self, c: Self|
                #[trigger] a.cmp_spec(b).eq() && #[trigger] b.cmp_spec(c).eq() ==> a.cmp_spec(
                    c,
                ).eq(),
            forall|a: Self, b: Self|
                #[trigger] a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt(),
            forall|a: Self, b: Self|
                #![auto] a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt(),
            forall|a: Self, b: Self, c: Self|
                #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(
                    c,
                ).lt(),
            forall|a: Self, b: Self, c: Self|
                #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).le() ==> a.cmp_spec(
                    c,
                ).lt(),
            forall|a: Self, b: Self, c: Self|
                #[trigger] a.cmp_spec(b).le() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(
                    c,
                ).lt();
}

pub enum Ordering {
    Less,
    Equal,
    Greater,
}

type ID = EndPoint;

pub trait VerusClone: Sized {}

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {

    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
            || (!self.k.is_None() && !other.k.is_None()
                && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }
}

// ===================== LOGICAL TESTS =====================

// SHOULD FAIL
// Test 1: Two different IDs both satisfy range_consistent vacuously — assert they must be equal.
// Since the range is empty, range_consistent holds for ANY id. This does NOT imply id1 == id2.
proof fn logical_test_vacuous_id_uniqueness<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id1: &ID,
    id2: &ID,
)
    requires
        lo.geq_spec(*hi),
{
    dm.empty_key_range_is_consistent(lo, hi, id1);
    dm.empty_key_range_is_consistent(lo, hi, id2);
    // Both range_consistent(lo, hi, id1) and range_consistent(lo, hi, id2) hold vacuously
    // But id1@ == id2@ is NOT entailed — they can be completely different
    assert(id1@ == id2@);
}

// SHOULD FAIL
// Test 2: Empty range consistency does NOT extend to a larger range.
// If lo >= hi (empty range), consistency for [lo2, hi) where lo2 < lo is NOT guaranteed.
proof fn logical_test_range_extension<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    lo2: &KeyIterator<K>,
    id: &ID,
)
    requires
        lo.geq_spec(*hi),
        lo2.lt_spec(*lo),
{
    dm.empty_key_range_is_consistent(lo, hi, id);
    // range_consistent(lo, hi, id) holds, but extending to [lo2, hi) is unjustified
    // lo2 < lo, so [lo2, hi) could contain keys not covered by the empty range
    assert(dm.range_consistent(lo2, hi, id));
}

// SHOULD FAIL
// Test 3: Empty range consistency does NOT imply the map is total (all keys have mappings).
// The spec says nothing about which keys are in the domain of the map.
proof fn logical_test_map_totality<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
)
    requires
        lo.geq_spec(*hi),
{
    dm.empty_key_range_is_consistent(lo, hi, id);
    // The vacuous range_consistent tells us nothing about the map's domain
    assert(forall|k: K| #[trigger] dm@.contains_key(k));
}

}
