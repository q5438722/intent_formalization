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
    pub open spec fn end_spec() -> (s: Self) {
        KeyIterator { k: None }
    }

    #[verifier(when_used_as_spec(end_spec))]
    pub fn end() -> (s: Self)
        ensures s.k.is_None()
    {
        KeyIterator { k: None }
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

    proof fn mind_the_gap(self)
        ensures
            forall|w, x, y, z| self.gap(w, x) && self.gap(y, z) && #[trigger] y.lt_spec(x) ==> #[trigger] self.gap(w, z),
            forall|w, x, y: KeyIterator<K>, z| #[trigger] self.gap(w, x) && y.geq_spec(w) && x.geq_spec(z) ==> #[trigger] self.gap(y, z),
            forall|l:KeyIterator<K>, k, m| #[trigger] self.gap(k, m) ==> !(k.lt_spec(l) && l.lt_spec(m) && #[trigger] self@.contains_key(*l.get()))
    {
        K::cmp_properties();
    }
}

type ID = EndPoint;

pub struct EndPoint {
    pub id: Vec<u8>,
}


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

    pub open spec fn lt_spec(self, other: Self) -> bool {
        (!self.k.is_None() && other.k.is_None())
      || (!self.k.is_None() && !other.k.is_None() && self.k.get_Some_0().cmp_spec(other.k.get_Some_0()).lt())
    }

    pub open spec fn geq_spec(self, other: Self) -> bool {
        !self.lt_spec(other)
    }
}

pub trait VerusClone : Sized {}

// ==================== BEHAVIORAL MUTATION TESTS ====================

// Test 1: Mutate postcondition 1 — drop the overlap condition (y < x).
// Original: gap(w,x) ∧ gap(y,z) ∧ y < x ⟹ gap(w,z)
// Mutated:  gap(w,x) ∧ gap(y,z) ⟹ gap(w,z) — without requiring overlap.
// Two non-overlapping gaps cannot be unconditionally merged.
// SHOULD FAIL
proof fn mutation_drop_overlap<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    w: KeyIterator<K>,
    x: KeyIterator<K>,
    y: KeyIterator<K>,
    z: KeyIterator<K>,
)
    requires
        map.gap(w, x),
        map.gap(y, z),
    ensures
        map.gap(w, z),
{
    map.mind_the_gap();
}

// Test 2: Mutate postcondition 2 — extend gap instead of narrowing.
// Original: gap(w,x) ∧ y ≥ w ∧ x ≥ z ⟹ gap(y,z) (subgap)
// Mutated:  gap(lo, hi) ∧ lo' < lo ⟹ gap(lo', hi) — extending left.
// A gap cannot be extended to cover a larger range.
// SHOULD FAIL
proof fn mutation_extend_gap_left<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    lo: KeyIterator<K>,
    hi: KeyIterator<K>,
    lo_prime: KeyIterator<K>,
)
    requires
        map.gap(lo, hi),
        lo_prime.lt_spec(lo),
    ensures
        map.gap(lo_prime, hi),
{
    map.mind_the_gap();
}

// Test 3: Negate postcondition 3 — assert key IS in the map despite gap.
// Original: gap(k,m) ∧ k < l < m ⟹ ¬contains_key(*l.get())
// Mutated:  gap(k,m) ∧ k < l < m ⟹ contains_key(*l.get())
// A key strictly inside a gap MUST be absent, not present.
// SHOULD FAIL
proof fn mutation_negate_membership<K: KeyTrait + VerusClone>(
    map: StrictlyOrderedMap<K>,
    k: KeyIterator<K>,
    m: KeyIterator<K>,
    l: KeyIterator<K>,
)
    requires
        map.gap(k, m),
        k.lt_spec(l),
        l.lt_spec(m),
        !l.is_end_spec(),
    ensures
        map@.contains_key(*l.get_spec()),
{
    map.mind_the_gap();
}

} // end verus!
