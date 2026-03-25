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
// BEHAVIORAL MUTATION TESTS - These should all FAIL verification
// ============================================================

// M1: After set(lo, hi, dst), a key IN [lo, hi) should map to dst@, not a wrong value
// SHOULD FAIL
proof fn test_mutation_in_range_wrong_value<K: KeyTrait + VerusClone>(
    dm_old: DelegationMap<K>, dm_new: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
    k: K, wrong: AbstractEndPoint,
)
    requires
        dm_old.valid(),
        dst.valid_physical_address(),
        dm_new.valid(),
        // Postconditions of set:
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm_new@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm_new@[*ki.get()] == dm_old@[*ki.get()],
        // k is in range
        KeyIterator::between(lo, KeyIterator::new_spec(k), hi),
        // wrong differs from dst
        wrong != dst,
{
    // Spec says dm_new@[k] == dst, but we assert the wrong value
    assert(dm_new@[k] == wrong);
}

// M2: After set, a key OUTSIDE [lo, hi) should be unchanged - assert it changed
// SHOULD FAIL
proof fn test_mutation_outside_range_changed<K: KeyTrait + VerusClone>(
    dm_old: DelegationMap<K>, dm_new: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
    k: K,
)
    requires
        dm_old.valid(),
        dst.valid_physical_address(),
        dm_new.valid(),
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm_new@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm_new@[*ki.get()] == dm_old@[*ki.get()],
        // k is outside the range
        !KeyIterator::between(lo, KeyIterator::new_spec(k), hi),
        // old value differs from dst
        dm_old@[k] != dst,
{
    // Spec says dm_new@[k] == dm_old@[k], but we assert it changed
    assert(dm_new@[k] != dm_old@[k]);
}

// M3: A valid DelegationMap should remain valid - asserting invalidity should fail
// SHOULD FAIL
proof fn test_mutation_result_invalid<K: KeyTrait + VerusClone>(
    dm: DelegationMap<K>,
)
    requires dm.valid(),
{
    // direct contradiction of the assumption
    assert(!dm.valid());
}

// M4: After set, the lo boundary key should map to dst, NOT the old value
// SHOULD FAIL
proof fn test_mutation_lo_maps_to_old<K: KeyTrait + VerusClone>(
    dm_old: DelegationMap<K>, dm_new: DelegationMap<K>,
    lo: KeyIterator<K>, hi: KeyIterator<K>, dst: AbstractEndPoint,
)
    requires
        dm_old.valid(),
        dst.valid_physical_address(),
        dm_new.valid(),
        !lo.is_end_spec(),
        lo.lt_spec(hi),
        forall |ki: KeyIterator<K>| #[trigger] KeyIterator::between(lo, ki, hi) ==> dm_new@[*ki.get()] == dst,
        forall |ki: KeyIterator<K>| !ki.is_end_spec() && !(#[trigger] KeyIterator::between(lo, ki, hi)) ==> dm_new@[*ki.get()] == dm_old@[*ki.get()],
        dm_old@[*lo.get()] != dst,
{
    K::cmp_properties();
    // lo is in [lo, hi): between(lo, lo, hi) holds because !lo.lt_spec(lo) && lo.lt_spec(hi)
    assert(KeyIterator::between(lo, lo, hi));
    // So dm_new@[*lo.get()] == dst, but we assert it equals the old value
    assert(dm_new@[*lo.get()] == dm_old@[*lo.get()]);
}

// M5: An address with empty id (length 0) IS valid - asserting it's not should fail
// SHOULD FAIL
proof fn test_mutation_empty_address_invalid() {
    let ep = AbstractEndPoint { id: Seq::empty() };
    // 0 < 0x100000 is true, so valid_physical_address() is true
    assert(!ep.valid_physical_address());
}

}
