use vstd::prelude::*;
use vstd::set_lib::*;
use vstd::assert_by_contradiction;

fn main() {}

verus! {

// ========== Type definitions from source ==========

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

spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool
{
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedVec<K> {

    pub closed spec fn view(self) -> Seq<K> {
        self.v@
    }

    pub closed spec fn valid(self) -> bool {
        sorted(self@) && self@.no_duplicates()
    }

    #[verifier::external_body]
    proof fn to_set(self) -> (s: Set<K>)
        requires self.valid(),
        ensures s == self@.to_set(),
                s.finite(),
                s.len() == self@.len(),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    fn len(&self) -> (len: usize)
        ensures len == self@.len()
    {
        unimplemented!()
    }

    #[verifier::external_body]
    fn index(&self, i: usize) -> (k: K)
        requires i < self@.len(),
        ensures k == self@[i as int]
    {
        unimplemented!()
    }

    #[verifier::external_body]
    fn erase(&mut self, start: usize, end: usize)
        requires
            old(self).valid(),
            start <= end <= old(self)@.len(),
        ensures
            self.valid(),
            self@ == old(self)@.subrange(0, start as int) + old(self)@.subrange(end as int, old(self)@.len() as int),
            old(self)@.to_set() == self@.to_set() + old(self)@.subrange(start as int, end as int).to_set(),
    {
        unimplemented!()
    }
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    pub open spec fn get_spec(&self) -> &K
        recommends self.k.is_some(),
    {
        &self.k.get_Some_0()
    }

    spec fn above_spec(&self, k: K) -> bool {
        self.k.is_None() || k.cmp_spec(self.k.get_Some_0()).lt()
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(above_spec))]
    fn above(&self, k: K) -> (b: bool)
        ensures b == self.above_spec(k),
    {
        unimplemented!()
    }

    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }

    pub open spec fn end_spec() -> (s: Self) {
        KeyIterator { k: None }
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(end_spec))]
    pub fn end() -> (s: Self)
        ensures s.k.is_None()
    {
        unimplemented!()
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

#[verifier::external_body]
pub fn vec_erase<A>(v: &mut Vec<A>, start: usize, end: usize)
    requires
        start <= end <= old(v).len(),
    ensures
        true,
        v@ == old(v)@.subrange(0, start as int) + old(v)@.subrange(end as int, old(v)@.len() as int),
{
    unimplemented!()
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
    proof fn gap_means_empty(self, lo:KeyIterator<K>, hi:KeyIterator<K>, k:KeyIterator<K>)
        requires
            self.gap(lo, hi),
            lo.lt_spec(k) && k.lt_spec(hi),
            self@.contains_key(*k.get()),
        ensures
            false,
    {
        unimplemented!()
    }

    #[verifier::external_body]
    proof fn choose_gap_violator(self, lo:KeyIterator<K>, hi:KeyIterator<K>) -> (r: KeyIterator<K>)
        requires
            !self.gap(lo, hi),
        ensures
            lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get()),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    fn erase(&mut self, lo: &KeyIterator<K>, hi: &KeyIterator<K>)
        requires
            old(self).valid(),
        ensures
            self.valid(),
            forall |k| {
                let ki = KeyIterator::new_spec(k);
                (if ki.geq_spec(*lo) && ki.lt_spec(*hi) {
                    !(#[trigger] self@.contains_key(k))
                } else {
                    (old(self)@.contains_key(k) ==>
                         self@.contains_key(k) && self@[k] == old(self)@[k])
                    && (self@.contains_key(k) ==> old(self)@.contains_key(k))
                })},
            forall |x, y| self.gap(x, y) <==> ({
                         ||| old(self).gap(x, y)
                         ||| (old(self).gap(x, *lo) &&
                              old(self).gap(*hi, y) &&
                              (hi.geq_spec(y) || hi.is_end_spec() || !self@.contains_key(*hi.get())))
                        }),
    {
        unimplemented!()
    }
}

pub struct EndPoint {
    pub id: Vec<u8>,
}

type ID = EndPoint;

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

pub enum Ordering {
    Less,
    Equal,
    Greater,
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
}

pub trait VerusClone : Sized {}

// ========== LOGICAL TESTS ==========

// Test 1: Assert that new gaps in the post-erase map must have existed before
// The gap postcondition allows NEW gaps created by erasing keys in [lo, hi).
// Asserting that a gap in the new map existed in the old map ignores the erase effect.
// SHOULD FAIL
proof fn test_no_new_gaps_after_erase<K: KeyTrait + VerusClone>(
    old_m: StrictlyOrderedMap<K>,
    new_m: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
)
    requires
        old_m.valid(),
        new_m.valid(),
        // Simulate erase postcondition (key membership)
        forall |kk: K| {
            let ki = KeyIterator::new_spec(kk);
            (if ki.geq_spec(lo) && ki.lt_spec(hi) {
                !(#[trigger] new_m@.contains_key(kk))
            } else {
                (old_m@.contains_key(kk) ==>
                     new_m@.contains_key(kk) && new_m@[kk] == old_m@[kk])
                && (new_m@.contains_key(kk) ==> old_m@.contains_key(kk))
            })
        },
        // Simulate erase postcondition (gap equivalence)
        forall |xx: KeyIterator<K>, yy: KeyIterator<K>| new_m.gap(xx, yy) <==> ({
                     ||| old_m.gap(xx, yy)
                     ||| (old_m.gap(xx, lo) &&
                          old_m.gap(hi, yy) &&
                          (hi.geq_spec(yy) || hi.is_end_spec() || !new_m@.contains_key(*hi.get_spec())))
                    }),
        // A gap exists in the new map
        new_m.gap(x, y),
{
    // Assert the gap must have existed in the old map (wrong: erase creates new gaps)
    assert(old_m.gap(x, y)); // SHOULD FAIL
}

// Test 2: Assert to_set returns empty set on a NON-EMPTY valid vec
// The spec says s.len() == self@.len(), so a non-empty vec must yield non-empty set.
// Asserting emptiness should fail.
// SHOULD FAIL
proof fn test_to_set_empty_on_nonempty<K: KeyTrait + VerusClone>(v: StrictlyOrderedVec<K>)
    requires
        v.valid(),
        v@.len() > 0,
{
    let s = v.to_set();
    assert(s.len() == 0); // SHOULD FAIL: s.len() == v@.len() > 0
}

// Test 3: Assert erase with non-empty range [lo, hi) preserves ALL keys (identity)
// When lo < hi, the range is non-empty and keys in the range should be erased.
// Asserting identity (map unchanged) should fail if any key exists in the range.
// Even without knowing whether keys exist, asserting equality for all keys is too strong.
// SHOULD FAIL
proof fn test_erase_identity_nonempty_range<K: KeyTrait + VerusClone>(
    old_m: StrictlyOrderedMap<K>,
    new_m: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    k: K,
)
    requires
        old_m.valid(),
        new_m.valid(),
        lo.lt_spec(hi),
        // Simulate erase postcondition
        forall |kk: K| {
            let ki = KeyIterator::new_spec(kk);
            (if ki.geq_spec(lo) && ki.lt_spec(hi) {
                !(#[trigger] new_m@.contains_key(kk))
            } else {
                (old_m@.contains_key(kk) ==>
                     new_m@.contains_key(kk) && new_m@[kk] == old_m@[kk])
                && (new_m@.contains_key(kk) ==> old_m@.contains_key(kk))
            })
        },
        // k is in the erased range AND was in old map
        KeyIterator::new_spec(k).geq_spec(lo),
        KeyIterator::new_spec(k).lt_spec(hi),
        old_m@.contains_key(k),
{
    // The key k should have been erased, but we assert it's still present
    // (testing that the spec correctly rejects identity after erase)
    assert(new_m@.contains_key(k)); // SHOULD FAIL
}

// Test 4: Assert KeyIterator total ordering without calling cmp_properties
// The KeyIterator ordering depends on K's cmp_spec, which requires cmp_properties()
// for properties like connectivity. Without it, trichotomy should NOT be provable.
// SHOULD FAIL
proof fn test_key_iterator_total_order_without_cmp_properties<K: KeyTrait + VerusClone>(
    a: KeyIterator<K>,
    b: KeyIterator<K>,
)
    requires
        !a.is_end_spec(),
        !b.is_end_spec(),
        !a.lt_spec(b),
        !(a == b),
{
    // Without calling K::cmp_properties(), Verus should not know that
    // !a.lt(b) && a != b implies b.lt(a) (connectivity/trichotomy)
    assert(b.lt_spec(a)); // SHOULD FAIL
}

} // end verus!
