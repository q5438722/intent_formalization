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

// ========== Behavioral Mutation Tests ==========
// These tests start from valid inputs but assert INCORRECT behaviors (mutated outputs).

// SHOULD FAIL: Negates the ensures of lemma_serialize_injective
// After calling the lemma with equal serializations, view_equal must hold — asserting its negation is wrong
proof fn test_mutation_negate_injective_ensures(a: u64, b: u64)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    assert(!a.view_equal(&b)); // Negation of ensures
}

// SHOULD FAIL: Asserts serialization is not self-equal (violates reflexivity of ==)
proof fn test_mutation_serialize_not_self_equal()
{
    let a: u64 = 7;
    assert(a.ghost_serialize() != a.ghost_serialize());
}

// SHOULD FAIL: Asserts tuple serialization uses reversed order (b ++ a instead of a ++ b)
proof fn test_mutation_pair_serialize_reversed_order()
{
    let p: (u64, u64) = (1u64, 2u64);
    assert(p.ghost_serialize() =~= p.1.ghost_serialize() + p.0.ghost_serialize());
}

// SHOULD FAIL: Negates the ensures of lemma_serialization_is_not_a_prefix_of
// After calling the lemma, the serializations must differ in the prefix — asserting equality is wrong
proof fn test_mutation_negate_prefix_ensures(a: u64, b: u64)
    requires
        !a.view_equal(&b),
        a.ghost_serialize().len() <= b.ghost_serialize().len(),
{
    a.lemma_serialization_is_not_a_prefix_of(&b);
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// SHOULD FAIL: Asserts that ghost_serialize produces an empty sequence for u64
// u64 serialization (little-endian bytes) should produce 8 bytes, not 0
proof fn test_mutation_serialize_empty()
{
    let a: u64 = 42;
    assert(a.ghost_serialize().len() == 0);
}

}
