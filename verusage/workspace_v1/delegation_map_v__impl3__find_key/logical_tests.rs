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
}

// Spec encoding of find_key's postcondition
pub open spec fn find_key_post<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>, k: K, o: Option<usize>
) -> bool {
    match o {
        None => !map@.contains_key(k),
        Some(i) => 0 <= i < map.keys@.len() && map.keys@[i as int] == k,
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

pub trait VerusClone : Sized {}

// ========== LOGICAL TESTS ==========

// Test 1: find_key returns None — assert map must be empty
// Spec only says the specific key is absent, not that the map is empty
// SHOULD FAIL
proof fn test_logical_none_implies_empty<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: K,
)
    requires
        map.valid(),
        find_key_post(map, k, Option::None),
{
    assert(map.keys@.len() == 0);
}

// Test 2: find_key returns Some(i) — assert i must be 0
// Spec doesn't constrain the index to any specific value
// SHOULD FAIL
proof fn test_logical_always_first<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: K,
    i: usize,
)
    requires
        map.valid(),
        find_key_post(map, k, Some(i)),
{
    assert(i == 0usize);
}

// Test 3: Two different keys found — assert they must be at adjacent indices
// Spec says nothing about relative positions of distinct keys
// SHOULD FAIL
proof fn test_logical_adjacent_indices<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k1: K,
    k2: K,
    i1: usize,
    i2: usize,
)
    requires
        map.valid(),
        k1 != k2,
        find_key_post(map, k1, Some(i1)),
        find_key_post(map, k2, Some(i2)),
{
    assert(i2 as int == i1 as int + 1 || i1 as int == i2 as int + 1);
}

}
