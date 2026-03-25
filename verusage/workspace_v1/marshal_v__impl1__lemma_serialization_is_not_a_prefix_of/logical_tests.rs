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

// ---- Logical Tests ----
// These test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: Assert ghost_serialize is non-deterministic.
// Same u64 value should always produce the same serialization,
// but this asserts they differ — testing determinism assumption.
proof fn test_logical_nondeterministic_serialization() {
    let a: u64 = 42;
    let b: u64 = 42;
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

// SHOULD FAIL: Assert u64 serialization has length 0.
// u64 LE serialization should be 8 bytes, not 0.
// Tests whether the spec constrains serialization length.
proof fn test_logical_zero_length_serialization() {
    let a: u64 = 0;
    assert(a.ghost_serialize().len() == 0);
}

// SHOULD FAIL: Assert one u64 serialization is strictly longer than another.
// Both u64 values serialize to 8 bytes, so lengths are always equal.
// This tests whether the spec allows reasoning about unequal lengths.
proof fn test_logical_unequal_serialization_lengths() {
    let a: u64 = 0;
    let b: u64 = 1;
    assert(a.ghost_serialize().len() > b.ghost_serialize().len());
}

}
