use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

pub enum Ordering {
    Less,
    Equal,
    Greater,
}

impl Ordering {
    pub open spec fn lt(self) -> bool {
        matches!(self, Ordering::Less)
    }
}

pub trait KeyTrait : Sized {
    spec fn cmp_spec(self, other: Self) -> Ordering;
}

pub trait VerusClone : Sized {}

struct StrictlyOrderedVec<K: KeyTrait> {
    v: Vec<K>,
}

spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool {
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

impl<K: KeyTrait + VerusClone> StrictlyOrderedVec<K> {
    pub closed spec fn view(self) -> Seq<K> {
        self.v@
    }

    pub closed spec fn valid(self) -> bool {
        sorted(self@) && self@.no_duplicates()
    }

    proof fn to_set(self) -> (s: Set<K>)
        requires self.valid(),
        ensures s == self@.to_set(),
                s.finite(),
                s.len() == self@.len(),
    {
        self@.unique_seq_to_set();
        self@.to_set()
    }
}

// ===== LOGICAL TESTS =====

// Test 1: cmp_spec has no transitivity axiom — should not be provable
// SHOULD FAIL
proof fn test_cmp_transitivity<K: KeyTrait>(a: K, b: K, c: K)
    requires a.cmp_spec(b).lt(), b.cmp_spec(c).lt(),
{
    assert(a.cmp_spec(c).lt());
}

// Test 2: cmp_spec has no antisymmetry axiom — should not be provable
// Even if a < b, we cannot conclude !(b < a) without an axiom
// SHOULD FAIL
proof fn test_cmp_antisymmetry<K: KeyTrait>(a: K, b: K)
    requires a.cmp_spec(b).lt(),
{
    assert(!b.cmp_spec(a).lt());
}

// Test 3: Two valid vecs with same length should NOT imply same set
// Same cardinality does not mean same elements
// SHOULD FAIL
proof fn test_same_length_implies_same_set<K: KeyTrait + VerusClone>(
    sov1: StrictlyOrderedVec<K>, sov2: StrictlyOrderedVec<K>)
    requires sov1.valid(), sov2.valid(), sov1@.len() == sov2@.len(),
{
    let s1 = sov1.to_set();
    let s2 = sov2.to_set();
    assert(s1 =~= s2);
}

}
