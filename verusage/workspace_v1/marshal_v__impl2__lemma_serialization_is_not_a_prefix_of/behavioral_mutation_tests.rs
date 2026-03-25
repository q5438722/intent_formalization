use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== Source definitions (from target file) ==========

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

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  {
    unimplemented!()
  }
}

// ========== BEHAVIORAL MUTATION TESTS ==========
// These tests start from valid inputs but assert INCORRECT output relationships.
// They should ALL FAIL verification.

// Test 1: Different u64 values — assert they have SAME serialization (mutated: ≠ → =)
// The lemma ensures they differ; this asserts they're equal.
// SHOULD FAIL
proof fn test_mutation_assert_same_serialization() {
    let a: u64 = 1;
    let b: u64 = 2;
    // Establish serialization length so preconditions hold
    lemma_auto_spec_u64_to_from_le_bytes();
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // Mutation: assert serializations are equal (contradicts ensures)
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// Test 2: Different u64 values — assert serialization IS a prefix (negate ensures directly)
// The lemma ensures NOT a prefix; this asserts IS a prefix.
// SHOULD FAIL
proof fn test_mutation_negate_ensures() {
    let a: u64 = 10;
    let b: u64 = 20;
    lemma_auto_spec_u64_to_from_le_bytes();
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // Mutation: negate the ensures clause — assert self IS a prefix of other
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// Test 3: Assert false after calling lemma — test if the axiom creates inconsistency
// If the spec is consistent, this SHOULD FAIL.
// SHOULD FAIL
proof fn test_mutation_assert_false_from_axiom() {
    let a: u64 = 100;
    let b: u64 = 200;
    lemma_auto_spec_u64_to_from_le_bytes();
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // If the ensures axiom were inconsistent, false would be derivable.
    // Since the spec should be consistent, this must fail.
    assert(false);
}

// Test 4: Assert view_equal for values that are NOT view_equal (mutated relationship)
// SHOULD FAIL
proof fn test_mutation_assert_view_equal_for_different() {
    let a: u64 = 5;
    let b: u64 = 10;
    // Mutation: assert view_equal when values differ
    assert(a.view_equal(&b));
}

}
