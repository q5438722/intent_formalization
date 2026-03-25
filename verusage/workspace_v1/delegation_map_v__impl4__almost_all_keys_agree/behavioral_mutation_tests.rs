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

	#[verifier::external_body]
    proof fn extend_range_consistent(self, x: &KeyIterator<K>, y: &KeyIterator<K>, z: &KeyIterator<K>, dst: &ID) 
        requires 
            self.range_consistent(x, y, dst),
            self.range_consistent(y, z, dst),
        ensures
            self.range_consistent(x, z, dst),
	{
		unimplemented!()
	}

	#[verifier::external_body]
    proof fn empty_key_range_is_consistent(&self, lo: &KeyIterator<K>, hi: &KeyIterator<K>, id: &ID)
        requires
            lo.geq_spec(*hi),
        ensures 
            self.range_consistent(lo, hi, id),
	{
		unimplemented!()
	}

    proof fn almost_all_keys_agree(&self, lo: usize, hi: usize, id: &ID)
        requires
            self.valid(),
            0 <= lo <= hi < self.lows.keys@.len(),
            forall |i| #![auto] lo <= i < hi ==> self.lows@[self.lows.keys@[i]]@ == id@,
        ensures
            self.range_consistent(&KeyIterator::new_spec(self.lows.keys@[lo as int]), &KeyIterator::new_spec(self.lows.keys@[hi as int]), id),
        decreases hi - lo,
    {
        let lo_k = self.lows.keys@[lo as int];
        let hi_k = self.lows.keys@[hi as int];
        let lo_ki = KeyIterator::new_spec(lo_k);
        let hi_ki = KeyIterator::new_spec(hi_k);
        if lo_ki.geq_spec(hi_ki) {
            self.empty_key_range_is_consistent(&lo_ki, &hi_ki, id);
        } else {
            assert(lo_ki.lt_spec(hi_ki) && lo < hi) by {
                K::cmp_properties();
            }
            let lo_next = (lo + 1) as usize;
            let lo_next_k = self.lows.keys@[lo_next as int];
            let lo_next_ki = KeyIterator::new_spec(lo_next_k);
            assert(self.lows.gap(lo_ki, lo_next_ki)) by {
                K::cmp_properties();
            }
            assert(self.range_consistent(&lo_ki, &lo_next_ki, id));
            self.almost_all_keys_agree(lo_next, hi, id);
            self.extend_range_consistent(&lo_ki, &lo_next_ki, &hi_ki, id);
        }

    }

}

type ID = EndPoint;

pub struct EndPoint {
    pub id: Vec<u8>,
}

impl EndPoint{

    pub open spec fn view(self) -> AbstractEndPoint {
        AbstractEndPoint{id: self.id@}
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

// ==================================================================
// BEHAVIORAL MUTATION TESTS: Valid call + assert mutated output
// ==================================================================

// Test 1: Assert range_consistent with a WRONG endpoint after valid call
// SHOULD FAIL
proof fn test_mutation_wrong_id<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, id: &ID, wrong_id: &ID)
    requires
        dm.valid(),
        dm.lows.keys@.len() >= 5,
        forall |i: int| #![auto] 0 <= i < 4 ==> dm.lows@[dm.lows.keys@[i]]@ == id@,
        id@ != wrong_id@,
{
    dm.almost_all_keys_agree(0, 4, id);
    // Postcondition: range_consistent for [keys[0], keys[4]) with id
    // Mutate: assert range_consistent with wrong_id instead
    assert(dm.range_consistent(
        &KeyIterator::new_spec(dm.lows.keys@[0]),
        &KeyIterator::new_spec(dm.lows.keys@[4]),
        wrong_id
    ));
}

// Test 2: Assert range_consistent for a WIDER range than guaranteed
// SHOULD FAIL
proof fn test_mutation_wider_range<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, id: &ID)
    requires
        dm.valid(),
        dm.lows.keys@.len() >= 6,
        forall |i: int| #![auto] 1 <= i < 4 ==> dm.lows@[dm.lows.keys@[i]]@ == id@,
{
    dm.almost_all_keys_agree(1, 4, id);
    // Postcondition: range_consistent for [keys[1], keys[4])
    // Mutate: assert for wider range [keys[0], keys[5])
    assert(dm.range_consistent(
        &KeyIterator::new_spec(dm.lows.keys@[0]),
        &KeyIterator::new_spec(dm.lows.keys@[5]),
        id
    ));
}

// Test 3: Assert the hi key's value equals id (hi key is EXCLUDED by `between`)
// SHOULD FAIL
proof fn test_mutation_hi_key_included<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, id: &ID)
    requires
        dm.valid(),
        dm.lows.keys@.len() >= 5,
        forall |i: int| #![auto] 0 <= i < 4 ==> dm.lows@[dm.lows.keys@[i]]@ == id@,
{
    dm.almost_all_keys_agree(0, 4, id);
    // Postcondition: range_consistent for [keys[0], keys[4])
    // keys[4] is NOT in the range (between uses strict < for upper bound)
    // Mutate: assert value at keys[4] equals id
    assert(dm@[dm.lows.keys@[4]] == id@);
}

}
