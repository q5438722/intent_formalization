use vstd::prelude::*;

fn main() {}

verus! {

pub enum Ordering {
    Less,
    Equal,
    Greater,
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

pub trait KeyTrait : Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;

    proof fn cmp_properties()
        ensures
        forall |a:Self, b:Self| #![auto] a == b <==> a.cmp_spec(b).eq(),
        forall |a:Self| #![auto] a.cmp_spec(a).eq(),
        forall |a:Self, b:Self| (#[trigger] a.cmp_spec(b)).eq() == b.cmp_spec(a).eq(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).eq() && #[trigger] b.cmp_spec(c).eq() ==> a.cmp_spec(c).eq(),
        forall |a:Self, b:Self|
            #[trigger] a.cmp_spec(b).lt() <==> b.cmp_spec(a).gt(),
        forall |a:Self, b:Self|
            #![auto] a.cmp_spec(b).ne() ==> a.cmp_spec(b).lt() || b.cmp_spec(a).lt(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).lt() && #[trigger] b.cmp_spec(c).le() ==> a.cmp_spec(c).lt(),
        forall |a:Self, b:Self, c:Self|
            #[trigger] a.cmp_spec(b).le() && #[trigger] b.cmp_spec(c).lt() ==> a.cmp_spec(c).lt();
}

pub trait VerusClone : Sized {}

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

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }

    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }
}

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

pub struct EndPoint {
    pub id: Vec<u8>,
}

impl EndPoint {
    pub open spec fn view(self) -> AbstractEndPoint {
        AbstractEndPoint { id: self.id@ }
    }
}

type ID = EndPoint;

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
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
}

// ============================================================
// LOGICAL TEST 1: Range non-emptiness
// range_consistent holds vacuously for empty ranges.
// It should NOT imply that the range is non-empty.
// ============================================================
// SHOULD FAIL
proof fn lt_range_nonemptiness<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    dst: ID,
)
    requires
        dm.range_consistent(&x, &y, &dst),
{
    K::cmp_properties();
    // SHOULD FAIL: range could be empty (x >= y)
    assert(x.lt_spec(y));
}

// ============================================================
// LOGICAL TEST 2: Destination uniqueness
// If range_consistent holds for two different destinations on
// the same (possibly empty) range, they need NOT be equal.
// ============================================================
// SHOULD FAIL
proof fn lt_destination_uniqueness<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    dst1: ID,
    dst2: ID,
)
    requires
        dm.range_consistent(&x, &y, &dst1),
        dm.range_consistent(&x, &y, &dst2),
{
    K::cmp_properties();
    // SHOULD FAIL: if the range is empty, both hold vacuously
    assert(dst1@ == dst2@);
}

// ============================================================
// LOGICAL TEST 3: Universal key mapping
// range_consistent for [x, y) should NOT imply that ALL keys
// (including those outside the range) map to dst.
// ============================================================
// SHOULD FAIL
proof fn lt_universal_mapping<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    dst: ID,
    k: K,
)
    requires
        dm.range_consistent(&x, &y, &dst),
{
    K::cmp_properties();
    // SHOULD FAIL: k may be outside [x, y)
    assert(dm@[k] == dst@);
}

}
