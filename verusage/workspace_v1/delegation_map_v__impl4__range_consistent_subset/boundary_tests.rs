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
// BOUNDARY TEST 1: Missing range_consistent precondition
// Without knowing the outer range is consistent, the inner
// range consistency should NOT be derivable.
// ============================================================
// SHOULD FAIL
proof fn bt_missing_range_consistent<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    x_inner: KeyIterator<K>,
    y_inner: KeyIterator<K>,
    dst: ID,
)
    requires
        // MISSING: dm.range_consistent(&x, &y, &dst),
        x_inner.geq_spec(x),
        !y.lt_spec(y_inner),
    ensures
        dm.range_consistent(&x_inner, &y_inner, &dst),
{
    K::cmp_properties();
}

// ============================================================
// BOUNDARY TEST 2: Inner lower bound below outer lower bound
// x_inner < x violates the subset relationship, so the
// postcondition should NOT be provable.
// ============================================================
// SHOULD FAIL
proof fn bt_inner_below_outer_lo<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    x_inner: KeyIterator<K>,
    y_inner: KeyIterator<K>,
    dst: ID,
)
    requires
        dm.range_consistent(&x, &y, &dst),
        x_inner.lt_spec(x),    // WRONG: x_inner < x instead of x_inner >= x
        !y.lt_spec(y_inner),
    ensures
        dm.range_consistent(&x_inner, &y_inner, &dst),
{
    K::cmp_properties();
}

// ============================================================
// BOUNDARY TEST 3: Inner upper bound above outer upper bound
// y_inner > y violates the subset relationship, so the
// postcondition should NOT be provable.
// ============================================================
// SHOULD FAIL
proof fn bt_inner_above_outer_hi<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    x_inner: KeyIterator<K>,
    y_inner: KeyIterator<K>,
    dst: ID,
)
    requires
        dm.range_consistent(&x, &y, &dst),
        x_inner.geq_spec(x),
        y.lt_spec(y_inner),     // WRONG: y < y_inner instead of y >= y_inner
    ensures
        dm.range_consistent(&x_inner, &y_inner, &dst),
{
    K::cmp_properties();
}

}
