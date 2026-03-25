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

// === Boundary Tests ===
// Each test violates a precondition (requires clause). SHOULD FAIL verification.

// SHOULD FAIL: two distinct u64 values are not view_equal
proof fn test_boundary_u64_distinct_values() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: two distinct usize values are not view_equal
proof fn test_boundary_usize_distinct_values() {
    let a: usize = 0usize;
    let b: usize = 100usize;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: u64 edge case — 0 vs max value, not view_equal
proof fn test_boundary_u64_zero_vs_max() {
    let a: u64 = 0u64;
    let b: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: Vec<u8> with different contents — not view_equal
proof fn test_boundary_vec_u8_different_contents() {
    let a_seq: Seq<u8> = Seq::empty();
    let b_seq: Seq<u8> = seq![1u8];
    assume(exists |a: Vec<u8>| a@ === a_seq);
    assume(exists |b: Vec<u8>| b@ === b_seq);
    let a: Vec<u8> = choose |a: Vec<u8>| a@ === a_seq;
    let b: Vec<u8> = choose |b: Vec<u8>| b@ === b_seq;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: lemma_fold_left_on_equiv_seqs with mismatched sequence lengths
proof fn test_boundary_fold_left_mismatched_lengths() {
    let s1: Seq<u64> = seq![1u64, 2u64];
    let s2: Seq<u64> = seq![1u64];
    let eq = |a: u64, b: u64| a@ === b@;
    let f = |acc: int, x: u64| acc + x.ghost_serialize().len();
    lemma_fold_left_on_equiv_seqs(s1, s2, eq, 0int, f);
}

}
