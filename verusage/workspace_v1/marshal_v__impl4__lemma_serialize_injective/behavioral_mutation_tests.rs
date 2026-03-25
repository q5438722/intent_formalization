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


// ========== Behavioral Mutation Tests ==========

// TEST 1: Call lemma_serialize_injective correctly, then negate the ensures
// Equal u64 values => same serialization => view_equal (by ensures)
// Assert NOT view_equal to contradict the postcondition
// SHOULD FAIL
proof fn test_mutation_negate_injectivity_ensures() {
    let a: u64 = 7u64;
    let b: u64 = 7u64;
    a.lemma_serialize_injective(&b);
    // The ensures says a.view_equal(&b), so asserting the opposite should fail
    assert(!a.view_equal(&b));
}

// TEST 2: Assert two different u64 values have equal serializations
// Mutates the expected relationship: different values should have different serializations
// SHOULD FAIL
proof fn test_mutation_different_values_same_serialization() {
    let a: u64 = 100u64;
    let b: u64 = 200u64;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// TEST 3: Call lemma_serialization_is_not_a_prefix_of correctly, then negate the ensures
// For two different u64 values, the lemma ensures serializations are not a prefix
// Assert they ARE a prefix to contradict the postcondition
// SHOULD FAIL
proof fn test_mutation_negate_prefix_ensures() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // Requires: !a.view_equal(&b) ✓ and a.ghost_serialize().len() <= b.ghost_serialize().len() ✓ (both 8)
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // The ensures says serializations are not a prefix-match; assert they are
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

}
