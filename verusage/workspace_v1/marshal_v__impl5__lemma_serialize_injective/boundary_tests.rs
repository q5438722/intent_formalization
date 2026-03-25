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

// ========== Boundary Tests ==========
// These tests violate preconditions and should be REJECTED by the verifier.

// SHOULD FAIL: Violates requires `self.ghost_serialize() == other.ghost_serialize()`
// Two distinct u64 values cannot have equal serializations
proof fn test_boundary_injective_different_u64_values()
{
    let a: u64 = 0;
    let b: u64 = 1;
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Violates requires `!self.view_equal(other)`
// Equal u64 values are view_equal, so the negation fails
proof fn test_boundary_prefix_on_equal_u64()
{
    let a: u64 = 42;
    let b: u64 = 42;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// SHOULD FAIL: Violates requires `self.ghost_serialize() == other.ghost_serialize()`
// Edge case: 0 and u64::MAX have maximally different serializations
proof fn test_boundary_injective_zero_and_max()
{
    let a: u64 = 0;
    let b: u64 = 0xFFFFFFFFFFFFFFFFu64;
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Violates requires `self.ghost_serialize() == other.ghost_serialize()`
// Different tuple values should not have equal serializations
proof fn test_boundary_pair_injective_different_values()
{
    let a: (u64, u64) = (1u64, 2u64);
    let b: (u64, u64) = (3u64, 4u64);
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Violates requires `self.ghost_serialize().len() <= other.ghost_serialize().len()`
// and `!self.view_equal(other)` — calling prefix lemma on equal tuple values
proof fn test_boundary_prefix_on_equal_pair()
{
    let a: (u64, u64) = (5u64, 10u64);
    let b: (u64, u64) = (5u64, 10u64);
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

}
