use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// === Definitions from source ===

#[verifier::external_body]
pub proof fn lemma_fold_left_on_equiv_seqs<A, B>(s1: Seq<A>, s2: Seq<A>, eq: spec_fn(A, A) -> bool, init: B, f: spec_fn(B, A) -> B)
    requires
      s1.len() == s2.len(),
      (forall |i: int| 0 <= i < s1.len() ==> eq(s1[i], s2[i])),
      (forall |b: B, a1: A, a2: A| #[trigger] eq(a1, a2) ==> #[trigger] f(b, a1) == f(b, a2)),
    ensures
      s1.fold_left(init, f) == s2.fold_left(init, f)
    decreases s1.len(),
{ unimplemented!() }

pub trait Marshalable : Sized {
    spec fn is_marshalable(&self) -> bool;

    #[verifier::external_body]
    spec fn ghost_serialize(&self) -> Seq<u8>
        recommends self.is_marshalable()
    { unimplemented!() }

    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
        requires
            self.view_equal(other),
        ensures
            self.is_marshalable() == other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize()
    { unimplemented!() }
}

impl Marshalable for u64 {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }
    open spec fn is_marshalable(&self) -> bool { true }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        spec_u64_to_le_bytes(*self)
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl Marshalable for usize {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }
    open spec fn is_marshalable(&self) -> bool {
        &&& *self as int <= u64::MAX
    }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (*self as u64).ghost_serialize()
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl Marshalable for Vec<u8> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }
    open spec fn is_marshalable(&self) -> bool {
        self@.len() <= usize::MAX &&
        (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
    }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (self@.len() as usize).ghost_serialize() + self@
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Vec<T> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        let s = self@;
        let o = other@;
        s.len() == o.len() && (forall |i: int| 0 <= i < s.len() ==> #[trigger] s[i].view_equal(&o[i]))
    }
    open spec fn is_marshalable(&self) -> bool {
        &&& self@.len() <= usize::MAX
        &&& (forall |x: T| self@.contains(x) ==> #[trigger] x.is_marshalable())
        &&& (self@.len() as usize).ghost_serialize().len() +
            self@.fold_left(0, |acc: int, x: T| acc + x.ghost_serialize().len()) <= usize::MAX
    }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (self@.len() as usize).ghost_serialize()
          + self@.fold_left(Seq::<u8>::empty(), |acc: Seq<u8>, x: T| acc + x.ghost_serialize())
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

// === Logical Tests ===
// Properties NOT explicitly guaranteed by the spec. These probe the semantic boundary.

// SHOULD FAIL: converse not guaranteed — equal serialization does NOT imply view_equal
// The spec only says view_equal => same serialization, not the reverse.
proof fn test_logical_serialize_equal_implies_view_equal() {
    assert(forall |x: u64, y: u64|
        x.ghost_serialize() =~= y.ghost_serialize() ==> x.view_equal(&y));
}

// SHOULD FAIL: the spec does NOT entail that all u64 values serialize identically
// ghost_serialize for u64 is spec_u64_to_le_bytes, which is injective
proof fn test_logical_all_u64_same_serialization() {
    assert(forall |x: u64, y: u64| x.ghost_serialize() =~= y.ghost_serialize());
}

// SHOULD FAIL: the spec does NOT guarantee that usize marshalability is unconditional
// is_marshalable for usize has guard (*self as int <= u64::MAX), so this should NOT be provable
// NOTE: If this PASSES, it reveals the guard is vacuous on this architecture (spec weakness)
proof fn test_logical_usize_always_marshalable() {
    assert(forall |x: usize| x.is_marshalable());
}

// SHOULD FAIL: non-view-equal values are NOT guaranteed to have different serialization
// The spec says nothing about what happens when view_equal is false
proof fn test_logical_non_view_equal_implies_different_serialize() {
    assert(forall |x: u64, y: u64|
        !x.view_equal(&y) ==> !(x.ghost_serialize() =~= y.ghost_serialize()));
}

// SHOULD FAIL: the spec does NOT guarantee that fold_left is commutative over eq
// lemma_fold_left_on_equiv_seqs requires element-wise equivalence, not arbitrary reordering
proof fn test_logical_fold_left_order_independent() {
    let s1: Seq<u64> = seq![1u64, 2u64];
    let s2: Seq<u64> = seq![2u64, 1u64];
    let f = |acc: Seq<u8>, x: u64| acc + x.ghost_serialize();
    // Asserting fold_left gives the same result regardless of order — NOT guaranteed
    assert(s1.fold_left(Seq::<u8>::empty(), f) =~= s2.fold_left(Seq::<u8>::empty(), f));
}

}
