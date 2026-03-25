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

impl Marshalable for Vec<u8> {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  open spec fn is_marshalable(&self) -> bool {
    self@.len() <= usize::MAX &&
    (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (self@.len() as usize).ghost_serialize()
      + self@
  }

  #[verifier::spinoff_prover]
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
  {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(self@ =~= self.ghost_serialize().subrange((self@.len() as usize).ghost_serialize().len() as int, self.ghost_serialize().len() as int));
    assert(other@ =~= other.ghost_serialize().subrange((other@.len() as usize).ghost_serialize().len() as int, other.ghost_serialize().len() as int));
  }
}

// ========== LOGICAL TESTS ==========

// Test 1: Assert serialization produces empty sequence — wrong length property
// The spec does not guarantee serialization length, but u64 serializes to 8 bytes, not 0
// SHOULD FAIL
proof fn test_logical_wrong_serialize_length(x: u64) {
    assert(x.ghost_serialize().len() == 0);
}

// Test 2: Assert all u64 values serialize identically — violates injectivity
// The spec guarantees same_serialize → view_equal, not that all serialize the same
// SHOULD FAIL
proof fn test_logical_all_u64_serialize_same(a: u64, b: u64) {
    assert(a.ghost_serialize() == b.ghost_serialize());
}

// Test 3: Assert wrong concrete serialization bytes for 0u64
// 0u64 in little-endian is [0,0,0,0,0,0,0,0], not [1,0,0,0,0,0,0,0]
// SHOULD FAIL
proof fn test_logical_wrong_concrete_bytes() {
    let x: u64 = 0u64;
    assert(x.ghost_serialize() =~= seq![1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
}

// Test 4: Assert Vec<u8> serialization is identity (no length prefix)
// ghost_serialize for Vec<u8> prepends the serialized length, so this is wrong
// SHOULD FAIL
proof fn test_logical_vec_serialize_is_identity(v: Vec<u8>)
  requires v.is_marshalable()
{
    assert(v.ghost_serialize() =~= v@);
}

// Test 5: Assert that u64 serialization length is 1 instead of 8
// spec_u64_to_le_bytes always produces 8 bytes
// SHOULD FAIL
proof fn test_logical_wrong_u64_serialize_length_one(x: u64) {
    assert(x.ghost_serialize().len() == 1);
}

}
