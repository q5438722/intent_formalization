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

// ---- Boundary Tests ----
// These tests violate preconditions (requires) and should be rejected.

// SHOULD FAIL: Calling lemma with two equal u64 values (zero).
// Violates the precondition !self.view_equal(other) since 0 == 0.
proof fn test_boundary_equal_u64_zero() {
    let a: u64 = 0;
    let b: u64 = 0;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// SHOULD FAIL: Calling lemma with two equal u64 values (max).
// Violates the precondition !self.view_equal(other) since MAX == MAX.
proof fn test_boundary_equal_u64_max() {
    let a: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    let b: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// SHOULD FAIL: Calling lemma with two equal usize values.
// Violates the precondition !self.view_equal(other) since 42 == 42.
proof fn test_boundary_equal_usize() {
    let a: usize = 42;
    let b: usize = 42;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

}
