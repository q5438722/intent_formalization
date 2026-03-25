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

// ===================== BOUNDARY TESTS =====================

// SHOULD FAIL
// Test 1: Call empty_key_range_is_consistent with lo < hi (precondition requires lo >= hi)
// This directly violates the requires clause.
proof fn boundary_test_lo_less_than_hi<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
)
    requires
        lo.lt_spec(*hi),  // lo < hi => geq_spec is false => precondition violated
{
    dm.empty_key_range_is_consistent(lo, hi, id);
}

// SHOULD FAIL
// Test 2: Call with lo = Some(k), hi = None (None is "infinity", so lo < hi)
// lt_spec(Some(k), None) is always true, so geq_spec is false.
proof fn boundary_test_some_vs_none<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    k: K,
    id: &ID,
)
{
    let lo = KeyIterator::<K> { k: Some(k) };
    let hi = KeyIterator::<K> { k: Option::None };
    // lo.lt_spec(hi) = true because (!self.k.is_None() && other.k.is_None())
    // So lo.geq_spec(hi) = false => precondition violated
    dm.empty_key_range_is_consistent(&lo, &hi, id);
}

// SHOULD FAIL
// Test 3: Call without any precondition (cannot prove lo >= hi)
// Verus must reject because it cannot establish the requires clause.
proof fn boundary_test_no_precondition<K: KeyTrait + VerusClone>(
    dm: &DelegationMap<K>,
    lo: &KeyIterator<K>,
    hi: &KeyIterator<K>,
    id: &ID,
)
{
    // No requires clause => lo.geq_spec(*hi) is unknown => precondition not provable
    dm.empty_key_range_is_consistent(lo, hi, id);
}

}
