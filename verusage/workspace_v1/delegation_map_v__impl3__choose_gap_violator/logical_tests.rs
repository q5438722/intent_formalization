use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl Ordering {

    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }
}

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
}

impl<K: KeyTrait + VerusClone> KeyIterator<K> {

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
    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    #[verifier(when_used_as_spec(is_end_spec))]
    pub fn is_end(&self) -> (b: bool)
        ensures b == self.is_end_spec()
    {
        matches!(self.k, None)
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

    spec fn gap(self, lo: KeyIterator<K>, hi: KeyIterator<K>) -> bool {
        forall |ki| lo.lt_spec(ki) && ki.lt_spec(hi) ==> !(#[trigger] self@.contains_key(*ki.get()))
    }

    proof fn choose_gap_violator(self, lo:KeyIterator<K>, hi:KeyIterator<K>) -> (r: KeyIterator<K>)
        requires
            !self.gap(lo, hi),
        ensures
            lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get()),
    {
        choose |r| #![auto] lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get_spec())
    }
}

#[verifier::reject_recursive_types(K)]
pub struct DelegationMap<K: KeyTrait + VerusClone> {
    lows: StrictlyOrderedMap<K>,
    m: Ghost<Map<K, AbstractEndPoint>>,

}


pub struct EndPoint {
    pub id: Vec<u8>,
}


type ID = EndPoint;

pub trait KeyTrait : Sized{
    spec fn cmp_spec(self, other: Self) -> Ordering;
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
    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }
}

pub trait VerusClone : Sized {}

// ========== LOGICAL TESTS ==========

// Test 1: Assert result must equal any arbitrary violator in the range
// (uniqueness of chosen violator is NOT guaranteed by spec)
// SHOULD FAIL
proof fn test_logical_unique_violator<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    other: KeyIterator<K>,
)
    requires
        !map.gap(lo, hi),
        lo.lt_spec(other) && other.lt_spec(hi) && map@.contains_key(*other.get_spec()),
{
    let r = map.choose_gap_violator(lo, hi);
    assert(r == other);
}

// Test 2: Assert result is less than an arbitrary midpoint
// (spec does not guarantee WHERE in (lo,hi) the result falls)
// SHOULD FAIL
proof fn test_logical_stronger_ordering<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    mid: KeyIterator<K>,
)
    requires
        !map.gap(lo, hi),
        lo.lt_spec(mid) && mid.lt_spec(hi),
{
    let r = map.choose_gap_violator(lo, hi);
    assert(r.lt_spec(mid));
}

// Test 3: Assert there is a gap between lo and the result
// (spec does not guarantee r is the FIRST key after lo)
// SHOULD FAIL
proof fn test_logical_gap_before_result<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
)
    requires !map.gap(lo, hi),
{
    let r = map.choose_gap_violator(lo, hi);
    assert(map.gap(lo, r));
}

}
