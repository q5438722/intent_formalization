use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// === SOURCE CODE (from target file) ===

// File: verus_extra/choose_v.rs
#[verifier::external_body]
pub proof fn choose_smallest(low: int, high: int, p: spec_fn(int)->bool) -> (res:int)
  requires
    exists |i:int| #![trigger(p(i))] low <= i < high && p(i),
  ensures
    low <= res < high,
    p(res),
    forall |i:int| #![trigger(p(i))] low <= i < res ==> !p(i),
  decreases
    high - low,
{
    unimplemented!()
}

// File: verus_extra/seq_lib_v.rs
#[verifier::external_body]
pub proof fn lemma_fold_left_on_equiv_seqs<A, B>(s1: Seq<A>, s2: Seq<A>, eq: spec_fn(A, A) -> bool, init: B, f: spec_fn(B, A) -> B)
    requires
      s1.len() == s2.len(),
      (forall |i: int| 0 <= i < s1.len() ==> eq(s1[i], s2[i])),
      (forall |b: B, a1: A, a2: A| #[trigger] eq(a1, a2) ==> #[trigger] f(b, a1) == f(b, a2)),
    ensures
      s1.fold_left(init, f) == s2.fold_left(init, f)
    decreases s1.len(),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn lemma_fold_left_append_merge<A, B>(s1: Seq<A>, s2: Seq<A>, f: spec_fn(A) -> Seq<B>)
  ensures
    (s1 + s2).fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      ==
    s1.fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      +
    s2.fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
  decreases
    s1.len() + s2.len()
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn some_differing_index_for_unequal_seqs<A>(s1: Seq<A>, s2: Seq<A>) -> (i: int)
  requires
    s1 != s2,
    s1.len() == s2.len(),
  ensures
    0 <= i < s1.len(),
    s1[i] != s2[i],
{
    unimplemented!()
}

// File: marshal_v.rs
pub trait Marshalable : Sized {

  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
    ensures self.view_equal(other) == other.view_equal(self)
  {unimplemented!()}

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
    requires
      !self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),
    ensures
      self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)
  {unimplemented!()}

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
    requires
      self.view_equal(other),
    ensures
      self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize()
  {unimplemented!()}

}


impl Marshalable for u64 {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
  {
    unimplemented!()
  }

  open spec fn is_marshalable(&self) -> bool {
    true
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  {
    unimplemented!()
  }

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  {
    unimplemented!()
  }

}


impl Marshalable for usize {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
  {
    unimplemented!()
  }

  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  {
    unimplemented!()
  }

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  {
    unimplemented!()
  }

}


// === LOGICAL TESTS ===
// These tests assert properties NOT explicitly guaranteed by the specification.
// They probe whether the spec allows unintended reasoning.

// Test 1: Try to prove serialization injectivity for u64 without calling any lemma.
// The spec only guarantees "not a prefix" via the lemma, not general injectivity.
// Without invoking the lemma, the verifier should not be able to prove this.
// SHOULD FAIL
proof fn test_logical_injectivity_without_lemma() {
    assert(forall |a: u64, b: u64| a@ != b@ ==> #[trigger] spec_u64_to_le_bytes(a) != #[trigger] spec_u64_to_le_bytes(b));
}

// Test 2: Try to prove that some_differing_index_for_unequal_seqs returns the FIRST
// differing index. The spec only guarantees it returns SOME differing index,
// not the smallest one. This stronger property is not entailed.
// SHOULD FAIL
proof fn test_logical_differing_index_is_first() {
    let s1: Seq<u8> = seq![1u8, 2u8, 3u8];
    let s2: Seq<u8> = seq![1u8, 9u8, 9u8];
    let i = some_differing_index_for_unequal_seqs(s1, s2);
    // The spec only guarantees 0 <= i < 3 and s1[i] != s2[i].
    // It does NOT guarantee i is the first such index.
    // Assert the stronger property: all indices before i are equal.
    assert(forall |j: int| 0 <= j < i ==> s1[j] == s2[j]);
}

// Test 3: Try to prove that non-view-equal u64 values always produce
// different-length serializations. This is false: all u64s serialize to 8 bytes.
// The spec does not guarantee different lengths for different values.
// SHOULD FAIL
proof fn test_logical_different_values_different_ser_lengths() {
    assert(forall |a: u64, b: u64| !a.view_equal(&b) ==>
        (#[trigger] a.ghost_serialize()).len() != (#[trigger] b.ghost_serialize()).len());
}

}
