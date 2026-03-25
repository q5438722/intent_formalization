use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ========== Source: trait and impls ==========

pub trait Marshalable : Sized {

  spec fn is_marshalable(&self) -> bool;

	#[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable(),
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
	{
		unimplemented!()
	}

	#[verifier::external_body]
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
	{
		unimplemented!()
	}

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
	{
		unimplemented!()
	}

	#[verifier::external_body]
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}


// ========== Boundary Tests ==========

// TEST 1: Call lemma_serialize_injective with different serializations
// Violates requires: self.ghost_serialize() == other.ghost_serialize()
// SHOULD FAIL
proof fn test_boundary_different_serializations() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    a.lemma_serialize_injective(&b);
}

// TEST 2: Call lemma_serialization_is_not_a_prefix_of on view_equal values
// Violates requires: !self.view_equal(other)
// SHOULD FAIL
proof fn test_boundary_view_equal_prefix() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// TEST 3: Edge case - call lemma_serialize_injective with 0 and u64::MAX
// Violates requires: different serializations at extreme boundary
// SHOULD FAIL
proof fn test_boundary_zero_and_max() {
    let a: u64 = 0u64;
    let b: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    a.lemma_serialize_injective(&b);
}

// TEST 4: Directly assert serialization equality on different values without any proof
// No lemma call - raw assertion that should be unprovable
// SHOULD FAIL
proof fn test_boundary_assert_serialize_equal_no_proof() {
    let a: u64 = 5u64;
    let b: u64 = 10u64;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

}
