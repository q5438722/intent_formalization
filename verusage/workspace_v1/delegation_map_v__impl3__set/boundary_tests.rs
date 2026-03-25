use vstd::prelude::*;
use vstd::assert_by_contradiction;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Type definitions (from target file) =====

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

pub trait VerusClone : Sized {
    fn clone(&self) -> (o: Self)
        ensures o == self;
}

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

    pub open spec fn get_spec(&self) -> &K
        recommends self.k.is_some(),
    {
        &self.k.get_Some_0()
    }

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(is_end_spec))]
    pub fn is_end(&self) -> (b: bool)
        ensures b == self.is_end_spec()
    {
        unimplemented!()
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(get_spec))]
    pub fn get(&self) -> (k: &K)
        requires !self.is_end(),
        ensures k == self.get_spec(),
    {
        unimplemented!()
    }
}

pub struct EndPoint {
    pub id: Vec<u8>,
}

type ID = EndPoint;

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
}

spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool {
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedVec<K> {
    pub closed spec fn view(self) -> Seq<K> { self.v@ }
    pub closed spec fn valid(self) -> bool { sorted(self@) && self@.no_duplicates() }
}

#[verifier::reject_recursive_types(K)]
struct StrictlyOrderedMap<K: KeyTrait + VerusClone> {
    keys: StrictlyOrderedVec<K>,
    vals: Vec<ID>,
    m: Ghost<Map<K, ID>>,
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedMap<K> {
    pub closed spec fn view(self) -> Map<K,ID> { self.m@ }

    pub closed spec fn map_valid(self) -> bool {
        &&& self.m@.dom().finite()
        &&& self.m@.dom() == self.keys@.to_set()
        &&& forall |i| 0 <= i < self.keys@.len() ==> #[trigger] (self.m@[self.keys@.index(i)]) == self.vals@.index(i)
    }

    pub closed spec fn valid(self) -> bool {
        &&& self.keys.valid()
        &&& self.keys@.len() == self.vals.len()
        &&& self.map_valid()
    }

    spec fn gap(self, lo: KeyIterator<K>, hi: KeyIterator<K>) -> bool {
        forall |ki| lo.lt_spec(ki) && ki.lt_spec(hi) ==> !(#[trigger] self@.contains_key(*ki.get()))
    }
}

// ========== BOUNDARY TESTS ==========

// B1: lt_spec should be irreflexive for concrete key iterators
// cmp_properties guarantees a.cmp_spec(a).eq(), so lt(k,k) is false
// SHOULD FAIL
proof fn test_boundary_lt_irreflexive<K: KeyTrait + VerusClone>(k: K) {
    K::cmp_properties();
    let ki = KeyIterator::new_spec(k);
    assert(ki.lt_spec(ki));
}

// B2: End iterator (None) is not less than itself
// lt_spec requires !self.k.is_None() in both disjuncts, which fails for None
// SHOULD FAIL
proof fn test_boundary_end_lt_end<K: KeyTrait + VerusClone>() {
    let end = KeyIterator::<K> { k: Option::None };
    assert(end.lt_spec(end));
}

// B3: End iterator (None) is not less than any concrete key
// End represents infinity; it is >= everything
// SHOULD FAIL
proof fn test_boundary_end_lt_concrete<K: KeyTrait + VerusClone>(k: K) {
    let end = KeyIterator::<K> { k: Option::None };
    let ki = KeyIterator::new_spec(k);
    assert(end.lt_spec(ki));
}

// B4: geq_spec is reflexive - k >= k should be true
// geq_spec(k,k) = !lt_spec(k,k) = !false = true
// SHOULD FAIL
proof fn test_boundary_geq_not_reflexive<K: KeyTrait + VerusClone>(k: K) {
    K::cmp_properties();
    let ki = KeyIterator::new_spec(k);
    assert(!ki.geq_spec(ki));
}

// B5: cmp_spec(k,k) returns Equal, not Less
// Ordering::Equal and Ordering::Less are distinct enum variants
// SHOULD FAIL
proof fn test_boundary_self_cmp_lt<K: KeyTrait + VerusClone>(k: K) {
    K::cmp_properties();
    assert(k.cmp_spec(k).lt());
}

}
