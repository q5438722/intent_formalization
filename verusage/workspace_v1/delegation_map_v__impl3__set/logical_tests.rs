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

    #[verifier::external_body]
    proof fn mind_the_gap(self)
        ensures
            forall|w, x, y, z| self.gap(w, x) && self.gap(y, z) && #[trigger] y.lt_spec(x) ==> #[trigger] self.gap(w, z),
            forall|w, x, y: KeyIterator<K>, z| #[trigger] self.gap(w, x) && y.geq_spec(w) && x.geq_spec(z) ==> #[trigger] self.gap(y, z),
            forall|l:KeyIterator<K>, k, m| #[trigger] self.gap(k, m) ==> !(k.lt_spec(l) && l.lt_spec(m) && #[trigger] self@.contains_key(*l.get()))
    {
        unimplemented!()
    }
}

// ========== LOGICAL TESTS ==========

// L1: set should be deterministic - two results from the same input must agree
// Asserting they differ should fail
// SHOULD FAIL
proof fn test_logical_not_deterministic<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>,
    new_map1: StrictlyOrderedMap<K>,
    new_map2: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
    query_k: K,
)
    requires
        old_map.valid(),
        new_map1.valid(),
        new_map1@ == old_map@.insert(k, v),
        new_map2.valid(),
        new_map2@ == old_map@.insert(k, v),
{
    // new_map1@ == old_map@.insert(k, v) == new_map2@
    // So new_map1@[query_k] == new_map2@[query_k] by transitivity
    assert(new_map1@[query_k] != new_map2@[query_k]);
}

// L2: Gaps can only be removed by set, never created
// Asserting a new gap appears where none existed should fail
// SHOULD FAIL
proof fn test_logical_gap_created<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
    lo: KeyIterator<K>, hi: KeyIterator<K>,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
        forall |l: KeyIterator<K>, h: KeyIterator<K>| new_map.gap(l, h) <==>
            old_map.gap(l, h)
            && !(l.lt_spec(KeyIterator::new_spec(k))
              && KeyIterator::new_spec(k).lt_spec(h)),
        // Old map did NOT have this gap
        !old_map.gap(lo, hi),
{
    // gap postcondition: new_gap <==> old_gap && (...)
    // old_gap is false, so new_gap must be false
    assert(new_map.gap(lo, hi));
}

// L3: set does NOT make all keys equal to v - only k maps to v
// Asserting an unrelated key maps to v should fail
// SHOULD FAIL
proof fn test_logical_all_keys_equal<K: KeyTrait + VerusClone>(
    old_map: StrictlyOrderedMap<K>, new_map: StrictlyOrderedMap<K>,
    k: K, v: EndPoint,
    k_other: K,
)
    requires
        old_map.valid(),
        new_map.valid(),
        new_map@ == old_map@.insert(k, v),
        k != k_other,
        old_map@.contains_key(k_other),
        old_map@[k_other] != v,
{
    // For k_other != k: new_map@[k_other] == old_map@[k_other] != v
    assert(new_map@[k_other] == v);
}

// L4: lt_spec is total for distinct non-end keys
// Asserting two distinct keys are incomparable should fail
// SHOULD FAIL
proof fn test_logical_lt_not_total<K: KeyTrait + VerusClone>(a: K, b: K)
    requires a != b,
{
    K::cmp_properties();
    let ka = KeyIterator::new_spec(a);
    let kb = KeyIterator::new_spec(b);
    // a != b ==> cmp_spec(a,b).ne() ==> a < b || b < a
    assert(!ka.lt_spec(kb) && !kb.lt_spec(ka));
}

// L5: mind_the_gap properties should be consistent
// Cannot derive false from gap axioms alone
// SHOULD FAIL
proof fn test_logical_mind_the_gap_unsound<K: KeyTrait + VerusClone>(
    m: StrictlyOrderedMap<K>,
)
    requires m.valid(),
{
    m.mind_the_gap();
    assert(false);
}

}
