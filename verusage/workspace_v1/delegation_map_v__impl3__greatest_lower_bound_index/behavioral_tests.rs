use vstd::prelude::*;

fn main() {}

verus! {

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

pub struct StrictlyOrderedVec<K: KeyTrait> {
    pub v: Vec<K>,
}

pub open spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool
{
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedVec<K> {

    pub open spec fn view(self) -> Seq<K> {
        self.v@
    }

    pub open spec fn valid(self) -> bool {
        sorted(self@) && self@.no_duplicates()
    }
}

pub struct EndPoint {
    pub id: Vec<u8>,
}

type ID = EndPoint;

#[verifier::reject_recursive_types(K)]
pub struct StrictlyOrderedMap<K: KeyTrait + VerusClone> {
    pub keys: StrictlyOrderedVec<K>,
    pub vals: Vec<ID>,
    pub m: Ghost<Map<K, ID>>,
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedMap<K> {

    pub open spec fn view(self) -> Map<K,ID> {
        self.m@
    }

    pub open spec fn map_valid(self) -> bool
    {
        &&& self.m@.dom().finite()
        &&& self.m@.dom() == self.keys@.to_set()
        &&& forall |i| 0 <= i < self.keys@.len()
                ==> #[trigger] (self.m@[self.keys@.index(i)]) == self.vals@.index(i)
    }

    pub open spec fn valid(self) -> bool {
        &&& self.keys.valid()
        &&& self.keys@.len() == self.vals.len()
        &&& self.map_valid()
    }

    pub open spec fn gap(self, lo: KeyIterator<K>, hi: KeyIterator<K>) -> bool {
        forall |ki| lo.lt_spec(ki) && ki.lt_spec(hi) ==> !(#[trigger] self@.contains_key(*ki.get()))
    }

    pub open spec fn greatest_lower_bound_spec(self, iter: KeyIterator<K>, glb: KeyIterator<K>) -> bool {
        (glb == iter || glb.lt_spec(iter)) &&
        (forall|k| KeyIterator::new_spec(k) != glb && #[trigger] self@.contains_key(k) && iter.above(k) ==> glb.above(k)) &&
        (!iter.is_end_spec() ==>
            glb.k.is_Some() &&
            self@.contains_key(glb.k.get_Some_0()) &&
            (exists|hi| #[trigger] self.gap(glb, hi) && #[trigger] KeyIterator::between(glb, iter, hi)))
    }
}

// Postcondition encoding
pub open spec fn glb_post<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>, iter: KeyIterator<K>, index: int
) -> bool {
    0 <= index < map.keys@.len() &&
    map.greatest_lower_bound_spec(iter, KeyIterator::new_spec(map.keys@[index]))
}

pub trait KeyTrait : Sized {
    spec fn zero_spec() -> Self where Self: std::marker::Sized;

    proof fn zero_properties()
        ensures
            forall |k:Self| k != Self::zero_spec() ==> (#[trigger] Self::zero_spec().cmp_spec(k)).lt();

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

    pub open spec fn end_spec() -> Self {
        KeyIterator { k: None }
    }

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    pub open spec fn get(&self) -> &K
        recommends self.k.is_some(),
    {
        &self.k.get_Some_0()
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn above(&self, k: K) -> bool {
        self.k.is_None() || k.cmp_spec(self.k.get_Some_0()).lt()
    }

    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
    }
}

pub trait VerusClone : Sized {}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutate ordering — assert GLB is strictly AFTER iter (negate glb <= iter)
// Postcondition guarantees glb <= iter; asserting the opposite should fail
// SHOULD FAIL
proof fn test_behavioral_glb_after_iter<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    iter: KeyIterator<K>,
    index: usize,
)
    requires
        map.valid(),
        map@.contains_key(K::zero_spec()),
        glb_post(map, iter, index as int),
        !iter.is_end_spec(),
{
    let glb = KeyIterator::new_spec(map.keys@[index as int]);
    // Negate: postcondition says glb <= iter, we assert iter < glb
    assert(iter.lt_spec(glb));
}

// Test 2: Mutate membership — assert GLB key is NOT in the map (when iter is non-end)
// Postcondition guarantees the GLB key is in the map for non-end iterators
// SHOULD FAIL
proof fn test_behavioral_glb_not_in_map<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    iter: KeyIterator<K>,
    index: usize,
)
    requires
        map.valid(),
        map@.contains_key(K::zero_spec()),
        glb_post(map, iter, index as int),
        !iter.is_end_spec(),
{
    assert(!map@.contains_key(map.keys@[index as int]));
}

// Test 3: Mutate "greatest" property — a key in the map below iter should be below GLB,
// but we assert it is NOT below GLB (negating the greatest lower bound property)
// SHOULD FAIL
proof fn test_behavioral_not_greatest<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    iter: KeyIterator<K>,
    index: usize,
    k: K,
)
    requires
        map.valid(),
        map@.contains_key(K::zero_spec()),
        glb_post(map, iter, index as int),
        !iter.is_end_spec(),
        map@.contains_key(k),
        iter.above(k),
        KeyIterator::new_spec(k) != KeyIterator::new_spec(map.keys@[index as int]),
{
    let glb = KeyIterator::new_spec(map.keys@[index as int]);
    // Negate: postcondition says k < glb for all such k; we assert k >= glb
    assert(!glb.above(k));
}

}
