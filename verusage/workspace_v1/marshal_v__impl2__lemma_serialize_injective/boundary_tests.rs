use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ========== Trait and Impl Definitions (from source) ==========

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable(),
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_serialize_injective(&self, other: &Self)
    requires
      self.is_marshalable(),
      other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
    ensures
      self.view_equal(other),
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
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
  {
    unimplemented!()
  }
}

// ========== BOUNDARY TESTS ==========

// Test 1: Violate precondition - two distinct u64 values have different serializations
// SHOULD FAIL
proof fn test_boundary_distinct_u64_values() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // ghost_serialize(0) != ghost_serialize(1), violating the requires clause
    a.lemma_serialize_injective(&b);
}

// Test 2: Violate precondition - edge case with u64::MAX vs 0
// SHOULD FAIL
proof fn test_boundary_u64_max_vs_zero() {
    let a: u64 = 0u64;
    let b: u64 = 0xFFFFFFFFFFFFFFFFu64;
    // Maximally different values, serializations clearly differ
    a.lemma_serialize_injective(&b);
}

// Test 3: Violate precondition - arbitrary u64 values without proving serialize equality
// SHOULD FAIL
proof fn test_boundary_arbitrary_u64_no_serialize_eq(a: u64, b: u64) {
    // No requires clause, so ghost_serialize equality is not established
    a.lemma_serialize_injective(&b);
}

// Test 4: Violate precondition - arbitrary usize values without proving is_marshalable
// SHOULD FAIL
proof fn test_boundary_arbitrary_usize_not_marshalable(a: usize, b: usize) {
    // is_marshalable not proven (requires *self as int <= u64::MAX)
    // ghost_serialize equality not proven
    a.lemma_serialize_injective(&b);
}

// Test 5: Assert view_equal on distinct concrete u64 values without calling lemma
// SHOULD FAIL
proof fn test_boundary_assert_view_equal_without_lemma() {
    let a: u64 = 42u64;
    let b: u64 = 99u64;
    // These are different values; view_equal (a@ === b@) should be false
    assert(a.view_equal(&b));
}

}
