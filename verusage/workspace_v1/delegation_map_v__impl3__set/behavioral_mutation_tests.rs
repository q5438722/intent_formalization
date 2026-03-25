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

// ========== BEHAVIORAL MUTATION TESTS ==========

// M1: After set(k, v), map[k] should be v
// Asserting it maps to a different value should fail
// SHOULD FAIL
proof fn test_mutation_wrong_value<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
{
    // Map::insert(k, v)[k] == v, so map[k] != v is false
    assert(new_map@[k] != v);
}

// M2: After set(k, v) for a new key, the map should change
// Asserting the map is unchanged should fail
// SHOULD FAIL
proof fn test_mutation_map_unchanged<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
        !old_map@.contains_key(k),
{
    // insert adds k to domain, so new_map@ != old_map@
    assert(new_map@ == old_map@);
}

// M3: After set(k, v), a gap containing k should be broken
// Asserting the gap persists should fail
// SHOULD FAIL
proof fn test_mutation_gap_unchanged<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
    lo_k: K, hi_k: K,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
        forall |l: KeyIterator<K>, h: KeyIterator<K>| new_map.gap(l, h) <==>
            old_map.gap(l, h)
            && !(l.lt_spec(KeyIterator::new_spec(k))
              && KeyIterator::new_spec(k).lt_spec(h)),
        // k is strictly between lo_k and hi_k
        lo_k.cmp_spec(k).lt(),
        k.cmp_spec(hi_k).lt(),
        // old map had a gap between lo_k and hi_k
        old_map.gap(KeyIterator::new_spec(lo_k), KeyIterator::new_spec(hi_k)),
{
    K::cmp_properties();
    let lo = KeyIterator::new_spec(lo_k);
    let hi = KeyIterator::new_spec(hi_k);
    // k is in (lo_k, hi_k), so the gap postcondition's second conjunct is false
    // new_map.gap(lo, hi) <==> true && false = false
    assert(new_map.gap(lo, hi));
}

// M4: After set, the result should be valid
// Asserting invalid should fail
// SHOULD FAIL
proof fn test_mutation_result_invalid<K: KeyTrait + VerusClone>(
    new_map: StrictlyOrderedMap<K>,
)
    requires
        new_map.valid(),
{
    // Directly contradicts the valid() assumption
    assert(!new_map.valid());
}

// M5: After set(k, v), keys other than k should NOT be newly added
// Asserting a spurious key exists should fail
// SHOULD FAIL
proof fn test_mutation_spurious_key<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
    k_prime: K,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
        k != k_prime,
        !old_map@.contains_key(k_prime),
{
    // insert(k, v) doesn't add k_prime (k != k_prime)
    assert(new_map@.contains_key(k_prime));
}

}
