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

// ===== BOUNDARY TESTS =====

// Test 1: Call to_set without establishing valid() precondition
// SHOULD FAIL
proof fn test_to_set_no_precondition<K: KeyTrait + VerusClone>(sov: StrictlyOrderedVec<K>)
{
    let s = sov.to_set();
}

// Test 2: Call to_set with explicitly negated valid() precondition
// SHOULD FAIL
proof fn test_to_set_negated_valid<K: KeyTrait + VerusClone>(sov: StrictlyOrderedVec<K>)
    requires !sov.valid(),
{
    let s = sov.to_set();
}

// Test 3: Valid empty vec should give empty set; assert non-empty
// SHOULD FAIL
proof fn test_empty_valid_nonempty_set<K: KeyTrait + VerusClone>(sov: StrictlyOrderedVec<K>)
    requires sov.valid(), sov@.len() == 0,
{
    let s = sov.to_set();
    assert(s.len() > 0);
}

}
