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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: find_key returns Some(i) — assert key NOT in the map (negate membership)
// The postcondition + map_valid should imply key IS in the map
// SHOULD FAIL
proof fn test_behavioral_some_but_not_in_map<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: K,
    i: usize,
)
    requires
        map.valid(),
        find_key_post(map, k, Some(i)),
{
    assert(!map@.contains_key(k));
}

// Test 2: find_key returns None — assert key IS in the map (negate absence)
// The postcondition directly says key is NOT in the map
// SHOULD FAIL
proof fn test_behavioral_none_but_in_map<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: K,
)
    requires
        map.valid(),
        find_key_post(map, k, Option::None),
{
    assert(map@.contains_key(k));
}

// Test 3: find_key returns Some(i) — assert keys[i] != k (negate key equality)
// The postcondition directly says keys[i] == k
// SHOULD FAIL
proof fn test_behavioral_wrong_key<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: K,
    i: usize,
)
    requires
        map.valid(),
        find_key_post(map, k, Some(i)),
{
    assert(map.keys@[i as int] != k);
}

}
