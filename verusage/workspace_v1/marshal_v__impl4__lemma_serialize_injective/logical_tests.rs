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


// ========== Logical Tests ==========

// TEST 1: Assert view_equal on arbitrary unequal u64 values without any supporting proof
// The spec does not guarantee this without equal serializations
// SHOULD FAIL
proof fn test_logical_view_equal_without_proof() {
    let a: u64 = 3u64;
    let b: u64 = 9u64;
    assert(a.view_equal(&b));
}

// TEST 2: Assert that ghost_serialize produces sequences of length > 16 for u64
// The spec says spec_u64_to_le_bytes produces 8 bytes, not > 16
// This tests whether the spec constrains serialization length
// SHOULD FAIL
proof fn test_logical_wrong_serialize_length() {
    let a: u64 = 42u64;
    assert(a.ghost_serialize().len() > 16);
}

// TEST 3: Converse of injectivity without proof -
// Try to prove that non-view-equal values have different serializations
// without calling any lemma. The spec does NOT directly state this as an ensures.
// SHOULD FAIL
proof fn test_logical_converse_injectivity_no_lemma() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // We know !a.view_equal(&b), but without calling any lemma,
    // we cannot prove their serializations differ
    assert(a.ghost_serialize() !== b.ghost_serialize());
}

}
