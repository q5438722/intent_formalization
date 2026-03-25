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


// === BOUNDARY TESTS ===
// These tests violate preconditions and should be rejected by the verifier.

// Test 1: Call lemma_serialization_is_not_a_prefix_of on equal u64 values.
// Violates precondition: !self.view_equal(other)
// Since both values are 42, view_equal is true, so !view_equal is false.
// SHOULD FAIL
proof fn test_boundary_equal_u64_not_prefix() {
    let a: u64 = 42;
    let b: u64 = 42;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Test 2: Call lemma_same_views_serialize_the_same on non-equal u64 values.
// Violates precondition: self.view_equal(other)
// Since 1 != 2, view_equal is false.
// SHOULD FAIL
proof fn test_boundary_different_u64_same_views() {
    let a: u64 = 1;
    let b: u64 = 2;
    a.lemma_same_views_serialize_the_same(&b);
}

// Test 3: Call choose_smallest with a predicate that is always false.
// Violates precondition: exists |i:int| low <= i < high && p(i)
// No i in [0, 10) satisfies false.
// SHOULD FAIL
proof fn test_boundary_choose_smallest_impossible() {
    let p = |i: int| false;
    let _ = choose_smallest(0, 10, p);
}

// Test 4: Call some_differing_index_for_unequal_seqs on two identical sequences.
// Violates precondition: s1 != s2
// Passing the same sequence for both arguments.
// SHOULD FAIL
proof fn test_boundary_equal_seqs_differing_index() {
    let s: Seq<u8> = seq![1u8, 2u8, 3u8];
    let _ = some_differing_index_for_unequal_seqs(s, s);
}

}
