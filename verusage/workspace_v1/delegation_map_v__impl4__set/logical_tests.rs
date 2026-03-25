use vstd::prelude::*;
use vstd::set_lib::*;
use vstd::assert_by_contradiction;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

impl AbstractEndPoint{
    pub open spec fn valid_physical_address(self) -> bool {
        self.id.len() < 0x100000
    }

}

impl Ordering{
    pub const fn is_lt(self) -> (b:bool)
        ensures b == self.lt(),
    {
        matches!(self, Ordering::Less)
    }

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

    pub open spec fn is_end_spec(&self) -> bool {
        self.k.is_None()
    }

    #[verifier(when_used_as_spec(is_end_spec))]
    pub fn is_end(&self) -> (b: bool)
        ensures b == self.is_end_spec()
    {
        matches!(self.k, None)
    }


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

	#[verifier::external_body]
   pub fn lt(&self, other: &Self) -> (b: bool)
        ensures b == self.lt_spec(*other),
	{
		unimplemented!()
	}

    spec fn above_spec(&self, k: K) -> bool {
        self.k.is_None() || k.cmp_spec(self.k.get_Some_0()).lt()
    }
    #[verifier(when_used_as_spec(above_spec))]
    fn above(&self, k: K) -> (b: bool)
        ensures b == self.above_spec(k),
    {
        self.is_end() || k.cmp(&self.k.as_ref().unwrap().clone()).is_lt()
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

	#[verifier::external_body]
    proof fn mind_the_gap(self)
        ensures
            forall|w, x, y, z| self.gap(w, x) && self.gap(y, z) && #[trigger] y.lt_spec(x) ==> #[trigger] self.gap(w, z),
            forall|w, x, y: KeyIterator<K>, z| #[trigger] self.gap(w, x) && y.geq_spec(w) && x.geq_spec(z) ==> #[trigger] self.gap(y, z),
            forall|l:KeyIterator<K>, k, m| #[trigger] self.gap(k, m) ==> !(k.lt_spec(l) && l.lt_spec(m) && #[trigger] self@.contains_key(*l.get()))
	{
		unimplemented!()
	}

	#[verifier::external_body]
    fn set(&mut self, k: K, v: ID)
        requires
            old(self).valid(),
        ensures
            self.valid(),
            self@ == old(self)@.insert(k, v),
            forall |lo, hi| self.gap(lo, hi) <==>
                            old(self).gap(lo, hi)
                        && !(lo.lt_spec(KeyIterator::new_spec(k))
                          && KeyIterator::new_spec(k).lt_spec(hi)),
	{
		unimplemented!()
	}

    spec fn greatest_lower_bound_spec(self, iter: KeyIterator<K>, glb: KeyIterator<K>) -> bool {
        (glb == iter || glb.lt_spec(iter)) &&
        (forall|k| KeyIterator::new_spec(k) != glb && #[trigger] self@.contains_key(k) && iter.above(k) ==> glb.above(k)) &&
        (!iter.is_end_spec() ==>
            glb.k.is_Some() &&
            self@.contains_key(glb.k.get_Some_0()) &&
            (exists|hi| #[trigger] self.gap(glb, hi) && #[trigger] KeyIterator::between(glb, iter, hi)))
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

	#[verifier::external_body]
    fn get_internal(&self, k: &K) -> (res: (ID, Ghost<KeyIterator<K>>))
        requires
            self.valid(),
        ensures ({
            let (id, glb) = res;
            &&& id@ == self@[*k]
            &&& self.lows.greatest_lower_bound_spec(KeyIterator::new_spec(*k), glb@)
            &&& id@.valid_physical_address()
    }),
	{
		unimplemented!()
	}

    #[verifier::external_body]
    pub fn set(&mut self, lo: &KeyIterator<K>, hi: &KeyIterator<K>, dst: &ID)
        requires
            old(self).valid(),
            dst@.valid_physical_address(),
        ensures
            self.valid(),
            forall |ki:KeyIterator<K>| #[trigger] KeyIterator::between(*lo, ki, *hi) ==> self@[*ki.get()] == dst@,
            forall |ki:KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(*lo, ki, *hi)) ==> self@[*ki.get()] == old(self)@[*ki.get()],
    {
        unimplemented!()
    }

}


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

    fn cmp(&self, other: &Self) -> (o: Ordering)
        requires true,
        ensures o == self.cmp_spec(*other);

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

pub trait VerusClone : Sized {
     fn clone(&self) -> (o: Self)
        ensures o == self;
}

type ID = EndPoint;


	#[verifier::external_body]
    pub fn clone_end_point(ep: &EndPoint) -> (cloned_ep: EndPoint)
        ensures
            cloned_ep@ == ep@
	{
		unimplemented!()
	}

// ============================================================
// LOGICAL TESTS - These should all FAIL verification
// ============================================================

// L1: Set should be deterministic - two results from same inputs must agree
// Asserting they differ should fail
// SHOULD FAIL
proof fn test_logical_not_deterministic<K: KeyTrait + VerusClone>(
    dm1: DelegationMap<K>, dm2: DelegationMap<K>,
    dm_old: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
    k: K,
)
    requires
        dm_old.valid(),
        dst.valid_physical_address(),
        dm1.valid(),
        dm2.valid(),
        // Same postconditions for dm1
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm1@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm1@[*ki.get()] == dm_old@[*ki.get()],
        // Same postconditions for dm2
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm2@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm2@[*ki.get()] == dm_old@[*ki.get()],
{
    // Both dm1 and dm2 should have the same value at k (deterministic)
    // Asserting they differ should fail
    assert(dm1@[k] != dm2@[k]);
}

// L2: With empty range (lo >= hi), all keys should be unchanged
// Asserting a key changed should fail
// SHOULD FAIL
proof fn test_logical_empty_range_changes<K: KeyTrait + VerusClone>(
    dm_old: DelegationMap<K>, dm_new: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
    k: K,
)
    requires
        dm_old.valid(),
        dst.valid_physical_address(),
        dm_new.valid(),
        // lo >= hi means empty range
        !lo.lt_spec(hi),
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm_new@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm_new@[*ki.get()] == dm_old@[*ki.get()],
{
    K::cmp_properties();
    // With lo >= hi, between(lo, ki, hi) is false for all ki
    // So second forall applies: dm_new@[k] == dm_old@[k]
    // Asserting they differ should fail
    assert(dm_new@[k] != dm_old@[k]);
}

// L3: A valid DelegationMap does NOT require all keys to map to the same endpoint
// Asserting this too-strong property should fail
// SHOULD FAIL
proof fn test_logical_valid_all_same<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>, k1: K, k2: K,
)
    requires
        dm.valid(),
        k1 != k2,
{
    // valid() only guarantees valid addresses, not identical values
    assert(dm@[k1] == dm@[k2]);
}

// L4: After set, NOT all keys map to dst - only keys in [lo, hi) do
// Asserting all keys map to dst is too strong
// SHOULD FAIL
proof fn test_logical_set_all_keys_to_dst<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
    k: K,
)
    requires
        dm.valid(),
        dst.valid_physical_address(),
        lo.lt_spec(hi),
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm@[*ki.get()] == dst,
{
    // Too strong: claiming ALL keys (not just those in range) map to dst
    assert(dm@[k] == dst);
}

// L5: lt_spec is total on non-end iterators - asserting two distinct
// non-end iterators are incomparable should fail
// SHOULD FAIL
proof fn test_logical_lt_not_total<K: KeyTrait + VerusClone>(a: K, b: K)
    requires a != b,
{
    K::cmp_properties();
    let ka = KeyIterator::new_spec(a);
    let kb = KeyIterator::new_spec(b);
    // Since a != b, by cmp_properties: a.cmp_spec(b).ne() ==> a < b || b < a
    // So at least one of ka.lt_spec(kb) or kb.lt_spec(ka) holds
    // Asserting NEITHER holds should fail
    assert(!ka.lt_spec(kb) && !kb.lt_spec(ka));
}

}
