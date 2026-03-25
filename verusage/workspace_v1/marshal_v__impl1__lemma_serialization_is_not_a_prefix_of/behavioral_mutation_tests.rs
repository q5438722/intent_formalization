use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;
use vstd::bytes::*;
use vstd::slice::*;

fn main() {}

verus!{

// ---- Trait and impl definitions (from source) ----

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
      self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),
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

  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
    // req, ens from trait
  {
    (*self as u64).lemma_serialization_is_not_a_prefix_of(&(*other as u64));
  }

}

// ---- Behavioral Mutation Tests ----
// These tests mutate expected outputs/relations. The spec should reject them.

// SHOULD FAIL: Assert two different u64 values have the same serialization.
// Mutated relation: 0u64 and 1u64 should serialize differently, not identically.
proof fn test_mutation_different_values_same_serialization() {
    let a: u64 = 0;
    let b: u64 = 1;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// SHOULD FAIL: Call lemma with valid inputs, then assert the negation of its postcondition.
// The lemma guarantees serialization is NOT a prefix; asserting it IS contradicts the result.
proof fn test_mutation_negate_postcondition() {
    let a: u64 = 0;
    let b: u64 = 1;
    // Establish the length precondition (both u64 serialize to 8 bytes)
    assume(a.ghost_serialize().len() <= b.ghost_serialize().len());
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // The postcondition tells us they are NOT prefix-equal.
    // Mutated: assert they ARE prefix-equal (contradicts postcondition).
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// SHOULD FAIL: Assert u64 MAX and 0 have the same serialization.
// These are maximally different values; their serializations should differ.
proof fn test_mutation_max_zero_same_serialization() {
    let a: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    let b: u64 = 0;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

}
