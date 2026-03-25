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

// ===================== BEHAVIORAL MUTATION TESTS =====================

// SHOULD FAIL
// Test 1: Negate the postcondition — assert that range_consistent does NOT hold
// After calling empty_key_range_is_consistent with valid precondition,
// the postcondition guarantees range_consistent. Negating it must fail.
proof fn behavioral_test_negate_postcondition<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
)
    requires
        lo.geq_spec(*hi),
{
    dm.empty_key_range_is_consistent(lo, hi, id);
    assert(!dm.range_consistent(lo, hi, id));  // contradicts ensures clause
}

// SHOULD FAIL
// Test 2: Assert that an arbitrary key maps to id after proving empty range consistency
// The empty range [lo, hi) has no keys, so range_consistent is vacuously true.
// It tells us nothing about the mapping of any specific key.
proof fn behavioral_test_arbitrary_key_maps_to_id<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
    k: K,
)
    requires
        lo.geq_spec(*hi),
{
    dm.empty_key_range_is_consistent(lo, hi, id);
    // range_consistent holds vacuously — no key is in [lo, hi)
    // So dm@[k] == id@ is NOT guaranteed for any k
    assert(dm@[k] == id@);
}

// SHOULD FAIL
// Test 3: Assert range_consistent for a non-empty range without any proof
// The spec only guarantees consistency when lo >= hi (empty range).
// For a non-empty range, it says nothing.
proof fn behavioral_test_nonempty_range_consistent<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
)
    requires
        lo.lt_spec(*hi),  // non-empty range
{
    assert(dm.range_consistent(lo, hi, id));
}

}
