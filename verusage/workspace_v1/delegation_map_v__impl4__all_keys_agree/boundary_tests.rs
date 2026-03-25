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

impl Ordering {
    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
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
#[verifier::spinoff_prover]

    #[verifier(when_used_as_spec(is_end_spec))]
    pub fn is_end(&self) -> (b: bool)
        ensures b == self.is_end_spec()
    {
        matches!(self.k, None)
    }


    pub open spec fn between(lhs: Self, ki: Self, rhs: Self) -> bool {
        !ki.lt_spec(lhs) && ki.lt_spec(rhs)
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

    pub closed spec fn map_valid(self) -> bool
    {
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

    proof fn all_keys_agree(&self, lo: usize, hi: usize, id: &ID)
        requires
            self.valid(),
            0 <= lo <= hi < self.lows.keys@.len(),
            forall |i| #![auto] lo <= i <= hi ==> self.lows@[self.lows.keys@[i]]@ == id@,
        ensures
            self.range_consistent(&KeyIterator::new_spec(self.lows.keys@[lo as int]), &KeyIterator::new_spec(self.lows.keys@[hi as int]), id),
        decreases hi - lo,
    {
        self.almost_all_keys_agree(lo, hi, id);
    }

	#[verifier::external_body]
    proof fn almost_all_keys_agree(&self, lo: usize, hi: usize, id: &ID)
        requires
            self.valid(),
            0 <= lo <= hi < self.lows.keys@.len(),
            forall |i| #![auto] lo <= i < hi ==> self.lows@[self.lows.keys@[i]]@ == id@,
        ensures
            self.range_consistent(&KeyIterator::new_spec(self.lows.keys@[lo as int]), &KeyIterator::new_spec(self.lows.keys@[hi as int]), id),
        decreases hi - lo,
	{
		unimplemented!()
	}
}

type ID = EndPoint;

pub struct EndPoint {
    pub id: Vec<u8>,
}

impl EndPoint {
    pub open spec fn view(self) -> AbstractEndPoint {
        AbstractEndPoint{id: self.id@}
    }
}


pub trait KeyTrait : Sized {
    spec fn zero_spec() -> Self where Self: std::marker::Sized;

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

    pub open spec fn new_spec(k: K) -> Self {
        KeyIterator { k: Some(k) }
    }

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }
}

pub trait VerusClone : Sized {}

// ============================================================
// BOUNDARY TESTS: Violate preconditions of all_keys_agree
// ============================================================

// Boundary Test 1: lo > hi violates the requires 0 <= lo <= hi
// SHOULD FAIL
proof fn test_boundary_lo_gt_hi<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, id: &ID)
    requires
        dm.valid(),
        dm.lows.keys@.len() >= 3,
{
    // lo=2, hi=1: violates lo <= hi
    dm.all_keys_agree(2, 1, id);
}

// Boundary Test 2: hi out of bounds (hi >= keys.len())
// SHOULD FAIL
proof fn test_boundary_hi_out_of_bounds<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, id: &ID)
    requires
        dm.valid(),
        dm.lows.keys@.len() == 2,
{
    // hi=2 but keys.len()==2, so hi is not < keys.len()
    assume(forall |i: int| #![auto] 0 <= i <= 2 ==> dm.lows@[dm.lows.keys@[i]]@ == id@);
    dm.all_keys_agree(0, 2, id);
}

// Boundary Test 3: missing self.valid() precondition
// SHOULD FAIL
proof fn test_boundary_not_valid<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, lo: usize, hi: usize, id: &ID)
    requires
        0 <= lo <= hi,
        // dm.valid() is intentionally omitted
{
    assume(hi < dm.lows.keys@.len());
    assume(forall |i: int| #![auto] lo <= i <= hi ==> dm.lows@[dm.lows.keys@[i]]@ == id@);
    dm.all_keys_agree(lo, hi, id);
}

// Boundary Test 4: missing forall condition (not all keys in range map to id)
// SHOULD FAIL
proof fn test_boundary_missing_forall<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, lo: usize, hi: usize, id: &ID)
    requires
        dm.valid(),
        0 <= lo <= hi,
        hi < dm.lows.keys@.len(),
        // forall condition intentionally omitted
{
    dm.all_keys_agree(lo, hi, id);
}

}
