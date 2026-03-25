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
// Logical Test 1: Sorted does NOT imply reverse is sorted
// Tests structural assumption: sortedness is NOT preserved under reversal
// SHOULD FAIL
// ============================================================
proof fn test_sorted_implies_reverse_sorted<K: KeyTrait>(s: Seq<K>)
    requires sorted(s), s.len() == 2,
{
    K::cmp_properties();
    let r = Seq::<K>::empty().push(s[1]).push(s[0]);
    assert(sorted(r));
}

// ============================================================
// Logical Test 2: Reflexive comparison is NOT Less
// Tests that cmp_spec(a, a) is Equal, not Less (irreflexivity of <)
// SHOULD FAIL
// ============================================================
proof fn test_reflexive_lt<K: KeyTrait>(a: K) {
    K::cmp_properties();
    assert(a.cmp_spec(a).lt());
}

// ============================================================
// Logical Test 3: Concatenation of two sorted seqs is NOT necessarily sorted
// Tests global assumption: sorted is not compositional over concatenation
// SHOULD FAIL
// ============================================================
proof fn test_concat_sorted<K: KeyTrait>(s: Seq<K>, t: Seq<K>)
    requires sorted(s), sorted(t), s.len() >= 1, t.len() >= 1,
{
    K::cmp_properties();
    assert(sorted(s + t));
}

// ============================================================
// Logical Test 4: Insert position is NOT always 0
// Tests determinism: the spec does not force insertion at position 0
// SHOULD FAIL
// ============================================================
proof fn test_insert_always_at_zero<K: KeyTrait>(
    old_seq: Seq<K>, k: K, new_seq: Seq<K>, idx: int
)
    requires
        sorted(old_seq),
        old_seq.no_duplicates(),
        !old_seq.contains(k),
        sorted(new_seq),
        new_seq.len() == old_seq.len() + 1,
        0 <= idx < new_seq.len(),
        new_seq == old_seq.insert(idx, k),
{
    assert(idx == 0);
}

// ============================================================
// Logical Test 5: Sorted elements are NOT all equal
// Tests stronger structural claim: sorted does not collapse distinct elements
// SHOULD FAIL
// ============================================================
proof fn test_sorted_implies_all_equal<K: KeyTrait>(s: Seq<K>)
    requires sorted(s), s.len() == 2,
{
    K::cmp_properties();
    assert(s[0] == s[1]);
}

}
