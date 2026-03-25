use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ========== Source Definitions ==========

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
    requires
      !self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),
    ensures
      self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)
  {unimplemented!()}

  #[verifier::external_body]
  proof fn lemma_serialize_injective(&self, other: &Self)
    requires
      self.is_marshalable(),
      other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
    ensures
      self.view_equal(other)
  {unimplemented!()}
}

impl Marshalable for u64 {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }
  open spec fn is_marshalable(&self) -> bool {
    true
  }
  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }
  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  { unimplemented!() }
  #[verifier::external_body]
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  { unimplemented!() }
  #[verifier::external_body]
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
  { unimplemented!() }
}

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
  }
  open spec fn is_marshalable(&self) -> bool {
    &&& self.0.is_marshalable()
    &&& self.1.is_marshalable()
    &&& self.0.ghost_serialize().len() + self.1.ghost_serialize().len() <= usize::MAX
  }
  open spec fn ghost_serialize(&self) -> Seq<u8> {
    self.0.ghost_serialize() + self.1.ghost_serialize()
  }
  #[verifier::external_body]
  #[verifier::spinoff_prover]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  { unimplemented!() }
  #[verifier::spinoff_prover]
  proof fn lemma_serialize_injective(self: &Self, other: &Self) {
    if !self.view_equal(other) {
      self.lemma_serialization_is_not_a_prefix_of(other);
      assert(other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)
             =~= other.ghost_serialize()); // OBSERVE
    }
  }
}

// ========== Logical Tests ==========
// These tests assert properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: Assert injectivity without calling the lemma (at generic trait level)
// The spec only provides this via the lemma call — it should not be automatically derivable
proof fn test_logical_injective_without_lemma<T: Marshalable>(a: T, b: T)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
{
    // Do NOT call lemma_serialize_injective
    assert(a.view_equal(&b)); // Not provable without the lemma
}

// SHOULD FAIL: Assert symmetry of view_equal at trait level
// The trait does not require view_equal to be symmetric
proof fn test_logical_view_equal_symmetric<T: Marshalable>(a: T, b: T)
    requires a.view_equal(&b)
{
    assert(b.view_equal(&a)); // Symmetry is not guaranteed by the trait
}

// SHOULD FAIL: Assert the contrapositive of injectivity without calling any lemma
// !view_equal(a,b) ==> ghost_serialize(a) != ghost_serialize(b) is NOT directly provable
proof fn test_logical_contrapositive_without_lemma(a: u64, b: u64)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        !a.view_equal(&b),
{
    assert(a.ghost_serialize() != b.ghost_serialize());
}

// SHOULD FAIL: Try to derive false from a valid lemma call (soundness check)
// If the spec is sound, calling the lemma with valid inputs should NOT enable proving false
proof fn test_logical_derive_false_after_lemma(a: u64)
    requires a.is_marshalable()
{
    a.lemma_serialize_injective(&a);
    // We now know a.view_equal(&a), which is true
    // But we should NOT be able to derive false
    assert(false);
}

// SHOULD FAIL: Assert that view_equal is transitive at the trait level
// The trait does not guarantee transitivity of view_equal
proof fn test_logical_view_equal_transitive<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.view_equal(&b),
        b.view_equal(&c),
{
    assert(a.view_equal(&c)); // Transitivity is not guaranteed by the trait
}

}
