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
// Behavioral Mutation Test 1: Length after insert is NOT unchanged
// Mutates postcondition: len == old_len + 1 → len == old_len
// SHOULD FAIL
// ============================================================
proof fn test_wrong_length_after_insert<K: KeyTrait>(
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
    assert(new_seq.len() == old_seq.len());
}

// ============================================================
// Behavioral Mutation Test 2: Inserted element k IS in the result set
// Mutates postcondition: k ∈ new_set → k ∉ new_set
// SHOULD FAIL
// ============================================================
proof fn test_k_not_in_result_set<K: KeyTrait>(
    old_seq: Seq<K>, k: K, new_seq: Seq<K>, idx: int
)
    requires
        sorted(old_seq),
        old_seq.no_duplicates(),
        !old_seq.contains(k),
        new_seq.len() == old_seq.len() + 1,
        0 <= idx < new_seq.len(),
        new_seq == old_seq.insert(idx, k),
        new_seq.to_set() == old_seq.to_set().insert(k),
{
    assert(!new_seq.to_set().contains(k));
}

// ============================================================
// Behavioral Mutation Test 3: Returned index is NOT at the length (out of bounds)
// Mutates postcondition: 0 <= i < len → i == len
// SHOULD FAIL
// ============================================================
proof fn test_index_equals_length<K: KeyTrait>(
    old_seq: Seq<K>, k: K, new_seq: Seq<K>, idx: int
)
    requires
        sorted(old_seq),
        old_seq.no_duplicates(),
        !old_seq.contains(k),
        new_seq.len() == old_seq.len() + 1,
        0 <= idx < new_seq.len(),
        new_seq == old_seq.insert(idx, k),
{
    assert(idx == new_seq.len());
}

// ============================================================
// Behavioral Mutation Test 4: Set IS changed (k was added)
// Mutates postcondition: new_set == old_set ∪ {k} → new_set == old_set
// SHOULD FAIL
// ============================================================
proof fn test_sets_unchanged_after_insert<K: KeyTrait>(
    old_seq: Seq<K>, k: K, new_seq: Seq<K>, idx: int
)
    requires
        sorted(old_seq),
        old_seq.no_duplicates(),
        !old_seq.contains(k),
        new_seq.len() == old_seq.len() + 1,
        0 <= idx < new_seq.len(),
        new_seq == old_seq.insert(idx, k),
        new_seq.to_set() == old_seq.to_set().insert(k),
{
    assert(new_seq.to_set() =~= old_seq.to_set());
}

// ============================================================
// Behavioral Mutation Test 5: Element at index IS k
// Mutates postcondition: new_seq[idx] == k → new_seq[idx] != k
// SHOULD FAIL
// ============================================================
proof fn test_element_not_at_index<K: KeyTrait>(
    old_seq: Seq<K>, k: K, new_seq: Seq<K>, idx: int
)
    requires
        sorted(old_seq),
        old_seq.no_duplicates(),
        !old_seq.contains(k),
        new_seq.len() == old_seq.len() + 1,
        0 <= idx < new_seq.len(),
        new_seq == old_seq.insert(idx, k),
{
    assert(new_seq[idx] != k);
}

}
