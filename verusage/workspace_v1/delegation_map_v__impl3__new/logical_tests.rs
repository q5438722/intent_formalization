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
// LOGICAL TEST 1: sorted implies no_duplicates
// sorted() only constrains cmp_spec ordering. Since cmp_spec is
// abstract (no axioms connecting it to value equality), sorted
// alone should NOT imply no_duplicates.
// ============================================================
// SHOULD FAIL
proof fn lt_sorted_implies_no_duplicates<K: KeyTrait>(s: Seq<K>)
    requires sorted(s), s.len() >= 2
{
    assert(s.no_duplicates());
}

// ============================================================
// LOGICAL TEST 2: cmp_spec transitivity not axiomatized
// KeyTrait provides no transitivity axiom for cmp_spec.
// Given a < b and b < c, we should NOT be able to derive a < c.
// ============================================================
// SHOULD FAIL
proof fn lt_cmp_transitivity<K: KeyTrait>(a: K, b: K, c: K)
    requires
        a.cmp_spec(b).lt(),
        b.cmp_spec(c).lt(),
{
    assert(a.cmp_spec(c).lt());
}

// ============================================================
// LOGICAL TEST 3: sorted does not bound sequence length
// sorted + no_duplicates impose no upper bound on length.
// Asserting a specific bound should fail.
// ============================================================
// SHOULD FAIL
proof fn lt_sorted_does_not_bound_length<K: KeyTrait>(s: Seq<K>)
    requires sorted(s), s.no_duplicates()
{
    assert(s.len() < 100);
}

}
