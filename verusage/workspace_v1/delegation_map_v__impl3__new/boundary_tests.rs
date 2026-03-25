use vstd::prelude::*;

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

pub open spec fn sorted<K: KeyTrait>(s: Seq<K>) -> bool
{
    forall |i, j| #![auto] 0 <= i < j < s.len() ==> s[i].cmp_spec(s[j]).lt()
}

// ============================================================
// BOUNDARY TEST 1: Reversed pair claimed as sorted
// a.cmp_spec(b) == Greater means a > b, so [a, b] is NOT
// sorted. The sorted spec requires strict Less for all i < j.
// ============================================================
// SHOULD FAIL
proof fn bt_reversed_pair_is_sorted<K: KeyTrait>(a: K, b: K)
    requires a.cmp_spec(b) == Ordering::Greater
{
    let s = Seq::empty().push(a).push(b);
    assert(sorted(s));
}

// ============================================================
// BOUNDARY TEST 2: Equal pair claimed as sorted
// a.cmp_spec(b) == Equal violates the strict Less requirement.
// [a, b] where a and b compare as Equal is NOT sorted.
// ============================================================
// SHOULD FAIL
proof fn bt_equal_pair_is_sorted<K: KeyTrait>(a: K, b: K)
    requires a.cmp_spec(b) == Ordering::Equal
{
    let s = Seq::empty().push(a).push(b);
    assert(sorted(s));
}

// ============================================================
// BOUNDARY TEST 3: Empty sequence has positive length
// new() ensures v@ == Seq::empty(), which has len() == 0.
// Asserting len > 0 violates this postcondition.
// ============================================================
// SHOULD FAIL
proof fn bt_empty_seq_has_positive_length<K: KeyTrait>() {
    let s: Seq<K> = Seq::empty();
    assert(s.len() > 0);
}

}
