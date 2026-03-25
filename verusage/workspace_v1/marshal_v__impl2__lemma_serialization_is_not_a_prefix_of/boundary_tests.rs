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

// ========== BOUNDARY TESTS ==========
// These tests violate preconditions of the lemma.
// They should ALL FAIL verification.

// Test 1: Equal u64 values — violates requires !self.view_equal(other)
// SHOULD FAIL
proof fn test_boundary_equal_u64_values() {
    let a: u64 = 42;
    let b: u64 = 42;
    // a.view_equal(&b) is true (42 === 42), so !view_equal is false
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Test 2: Zero u64 values — violates requires !self.view_equal(other) at edge case 0
// SHOULD FAIL
proof fn test_boundary_zero_u64_values() {
    let a: u64 = 0;
    let b: u64 = 0;
    // Edge case: both are zero, view_equal is true
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Test 3: Equal usize values — violates requires !self.view_equal(other)
// SHOULD FAIL
proof fn test_boundary_equal_usize_values() {
    let a: usize = 100;
    let b: usize = 100;
    // view_equal for usize: self@ === other@ → 100 === 100 → true
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Test 4: Assert the ensures conclusion for equal values without calling the lemma
// The conclusion should be FALSE for equal values (they have identical serializations)
// SHOULD FAIL
proof fn test_boundary_conclusion_for_equal_values() {
    let a: u64 = 7;
    let b: u64 = 7;
    // Without calling lemma, directly assert the conclusion for equal values
    // Since a == b, ghost_serialize(a) == ghost_serialize(b), so
    // a.ghost_serialize() == b.ghost_serialize().subrange(0, len) — conclusion is false
    assert(a.ghost_serialize() != b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// Test 5: u64::MAX equal values — edge case at maximum
// SHOULD FAIL
proof fn test_boundary_max_u64_equal() {
    let a: u64 = u64::MAX;
    let b: u64 = u64::MAX;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

}
