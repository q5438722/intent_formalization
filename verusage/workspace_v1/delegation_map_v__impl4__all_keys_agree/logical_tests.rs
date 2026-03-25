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
// LOGICAL TESTS: Properties NOT explicitly guaranteed by spec
// ============================================================

// Logical Test 1: Arbitrary key maps to id (global agreement)
// The postcondition only guarantees keys BETWEEN lo and hi.
// An arbitrary key k outside the range should NOT be guaranteed to map to id.
// SHOULD FAIL
proof fn test_logical_global_agreement<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, lo: usize, hi: usize, id: &ID, k: K)
    requires
        dm.valid(),
        0 <= lo < hi,
        hi < dm.lows.keys@.len(),
        forall |i: int| #![auto] lo <= i <= hi ==> dm.lows@[dm.lows.keys@[i]]@ == id@,
{
    dm.all_keys_agree(lo, hi, id);
    // Try to assert that an ARBITRARY key (not necessarily in the range) maps to id.
    // This is a strictly stronger property than what range_consistent guarantees.
    assert(dm@[k] == id@);
}

// Logical Test 2: Weaker precondition suffices (only first key maps to id)
// The real precondition requires ALL keys in [lo, hi] to map to id.
// Try using only the first key and deriving the full range_consistent.
// SHOULD FAIL
proof fn test_logical_weaker_precondition<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, lo: usize, hi: usize, id: &ID)
    requires
        dm.valid(),
        0 <= lo < hi,
        hi < dm.lows.keys@.len(),
        // Only the first key maps to id (weaker than forall lo..=hi)
        dm.lows@[dm.lows.keys@[lo as int]]@ == id@,
{
    // Try to derive range_consistent from a single key agreement — should not suffice.
    assert(dm.range_consistent(&KeyIterator::new_spec(dm.lows.keys@[lo as int]), &KeyIterator::new_spec(dm.lows.keys@[hi as int]), id));
}

// Logical Test 3: Single key agreement does not extend to the next key
// Knowing dm@[keys[lo]] == id@ does not imply dm@[keys[lo+1]] == id@
// SHOULD FAIL
proof fn test_logical_single_key_no_extend<K: KeyTrait + VerusClone>(dm: &DelegationMap<K>, lo: usize, id: &ID)
    requires
        dm.valid(),
        lo + 1 < dm.lows.keys@.len(),
        dm@[dm.lows.keys@[lo as int]] == id@,
{
    // Asserting the next key also maps to id — not guaranteed by the spec.
    assert(dm@[dm.lows.keys@[lo as int + 1]] == id@);
}

}
