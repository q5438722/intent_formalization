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
    spec fn zero_spec() -> Self where Self: std::marker::Sized;
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

    fn cmp(&self, other: &Self) -> (o: Ordering)
        requires true,
        ensures o == self.cmp_spec(*other);
}

pub trait VerusClone : Sized {}

pub struct KeyIterator<K: KeyTrait + VerusClone> {
    pub k: Option<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {
    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn end_spec() -> (s: Self) {
        KeyIterator { k: None }
    }

    #[verifier(when_used_as_spec(end_spec))]
    pub fn end() -> (s: Self)
        ensures s.k.is_None()
    {
        KeyIterator { k: None }
    }

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    #[verifier(when_used_as_spec(is_end_spec))]
    pub fn is_end(&self) -> (b: bool)
        ensures b == self.is_end_spec()
    {
        matches!(self.k, None)
    }

    pub open spec fn get_spec(&self) -> &K
        recommends self.k.is_some(),
    {
        &self.k.get_Some_0()
    }

    #[verifier(when_used_as_spec(get_spec))]
    pub fn get(&self) -> (k: &K)
        requires !self.is_end(),
        ensures k == self.get_spec(),
    {
        self.k.as_ref().unwrap()
    }

    pub open spec fn cmp_spec(self, other: Self) -> Ordering {
        match (self.k, other.k) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(i), Some(j)) => { i.cmp_spec(j) }
        }
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

pub struct EndPoint {
    pub id: Vec<u8>,
}

impl EndPoint {
    pub open spec fn view(self) -> AbstractEndPoint {
        AbstractEndPoint{id: self.id@}
    }
}

type ID = EndPoint;

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
}

spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool {
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedVec<K> {
    pub closed spec fn view(self) -> Seq<K> {
        self.v@
    }
    pub closed spec fn valid(self) -> bool {
        sorted(self@) && self@.no_duplicates()
    }
}

#[verifier::reject_recursive_types(K)]
struct StrictlyOrderedMap<K: KeyTrait + VerusClone> {
    keys: StrictlyOrderedVec<K>,
    vals: Vec<ID>,
    m: Ghost<Map<K, ID>>,
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedMap<K> {
    pub closed spec fn view(self) -> Map<K,ID> {
        self.m@
    }
    pub closed spec fn valid(self) -> bool {
        &&& self.keys.valid()
        &&& self.keys@.len() == self.vals.len()
        &&& self.m@.dom().finite()
        &&& self.m@.dom() == self.keys@.to_set()
        &&& forall |i| 0 <= i < self.keys@.len() ==> #[trigger] (self.m@[self.keys@.index(i)]) == self.vals@.index(i)
    }
    spec fn gap(self, lo: KeyIterator<K>, hi: KeyIterator<K>) -> bool {
        forall |ki| lo.lt_spec(ki) && ki.lt_spec(hi) ==> !(#[trigger] self@.contains_key(*ki.get()))
    }
}

#[verifier::reject_recursive_types(K)]
pub struct DelegationMap<K: KeyTrait + VerusClone> {
    lows: StrictlyOrderedMap<K>,
    m: Ghost<Map<K, AbstractEndPoint>>,
}

impl<K: KeyTrait + VerusClone> DelegationMap<K> {
    pub closed spec fn view(self) -> Map<K,AbstractEndPoint> {
        self.m@
    }

    pub closed spec fn valid(self) -> bool {
        &&& self.lows.valid()
        &&& self.lows@.contains_key(K::zero_spec())
        &&& self@.dom().is_full()
        &&& (forall|k| #[trigger] self@[k].valid_physical_address())
        &&& (forall|k, i, j|
                      self.lows@.contains_key(i)
                   && self.lows.gap(KeyIterator::new_spec(i), j)
                   && #[trigger] KeyIterator::between(KeyIterator::new_spec(i), KeyIterator::new_spec(k), j)
                   ==> self@[k] == self.lows@[i]@)
    }

    pub open spec fn range_consistent(self, lo: &KeyIterator<K>, hi: &KeyIterator<K>, dst: &ID) -> bool {
        forall |k| KeyIterator::between(*lo, KeyIterator::new_spec(k), *hi) ==> (#[trigger] self@[k]) == dst@
    }
}

// ========================================================================
// LOGICAL TESTS
// These test properties NOT explicitly guaranteed by the specification:
// determinism, stronger inequalities, structural assumptions, cross-function misuse.
// Each test should FAIL verification if the specification is correct.
// ========================================================================

// Test 1: The hi boundary is EXCLUDED from range_consistent (half-open interval [lo, hi)).
// Asserting dm@[*hi.get()] == dst@ should fail because hi is not in the range.
// SHOULD FAIL
proof fn test_hi_boundary_included<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    dst: ID
)
    requires
        dm.valid(),
        dm.range_consistent(&lo, &hi, &dst),
        lo.lt_spec(hi),
        !hi.is_end_spec(),
{
    assert(dm@[*hi.get_spec()] == dst@); // SHOULD FAIL
}

// Test 2: Two valid DelegationMaps do NOT have to agree on all keys.
// valid() constrains internal structure but not the specific mapping.
// SHOULD FAIL
proof fn test_two_valid_maps_agree<K: KeyTrait + VerusClone>(
    dm1: DelegationMap<K>,
    dm2: DelegationMap<K>,
    k: K
)
    requires
        dm1.valid(),
        dm2.valid(),
{
    assert(dm1@[k] == dm2@[k]); // SHOULD FAIL
}

// Test 3: lt_spec and cmp_spec on KeyIterator DISAGREE about the ordering of None vs Some.
// In lt_spec: Some(x) < None (end is maximum).
// In cmp_spec: (Some(_), None) => Greater, so Some > None (end is minimum).
// Asserting consistency between them should FAIL.
// SHOULD FAIL
proof fn test_lt_cmp_inconsistency<K: KeyTrait + VerusClone>(
    ki: KeyIterator<K>
)
    requires
        !ki.is_end_spec(),
{
    let end = KeyIterator::<K>::end_spec();
    // lt_spec says ki < end (true), but cmp_spec says ki > end (Greater.lt() = false)
    assert(ki.cmp_spec(end).lt() == ki.lt_spec(end)); // SHOULD FAIL
}

} // verus!
