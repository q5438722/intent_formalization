use vstd::prelude::*;

fn main() {}

verus! {

pub enum Ordering {
    Less,
    Equal,
    Greater,
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

spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool
{
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

// ============================================================
// Boundary Test 1: Ordering::Less is NOT Equal
// SHOULD FAIL
// ============================================================
proof fn test_less_is_equal() {
    assert(Ordering::Less.eq());
}

// ============================================================
// Boundary Test 2: Ordering::Greater is NOT less-or-equal
// SHOULD FAIL
// ============================================================
proof fn test_greater_is_le() {
    assert(Ordering::Greater.le());
}

// ============================================================
// Boundary Test 3: Ordering::Equal is NOT less-than
// SHOULD FAIL
// ============================================================
proof fn test_equal_is_lt() {
    assert(Ordering::Equal.lt());
}

// ============================================================
// Boundary Test 4: An unsorted pair [a, b] where b < a is NOT sorted
// SHOULD FAIL
// ============================================================
proof fn test_unsorted_pair_is_sorted<K: KeyTrait>(a: K, b: K)
    requires b.cmp_spec(a).lt(),
{
    K::cmp_properties();
    let s = Seq::<K>::empty().push(a).push(b);
    assert(sorted(s));
}

// ============================================================
// Boundary Test 5: A sequence with duplicate elements [a, a] is NOT strictly sorted
// SHOULD FAIL
// ============================================================
proof fn test_equal_elements_sorted<K: KeyTrait>(a: K) {
    K::cmp_properties();
    let s = Seq::<K>::empty().push(a).push(a);
    assert(sorted(s));
}

}
