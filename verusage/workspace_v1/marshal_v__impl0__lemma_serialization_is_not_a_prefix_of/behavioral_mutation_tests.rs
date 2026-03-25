use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;
use vstd::bytes::*;
use vstd::slice::*;

fn main() {}

verus!{

// === Source definitions (trait + impls) ===

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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self) {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int) =~= other.ghost_serialize());
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self) {
    unimplemented!()
  }
}

// === Behavioral Mutation Tests ===

// Mutation Test 1: Assert two different u64 values have EQUAL serializations
// This mutates the expected behavior — different values must have different serializations
proof fn test_mutation_equal_serialization() // SHOULD FAIL
{
    let a: u64 = 1;
    let b: u64 = 2;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// Mutation Test 2: Assert serialization of a IS a prefix of b (opposite of the postcondition)
// The lemma ensures serialization is NOT a prefix; here we assert it IS
proof fn test_mutation_is_prefix() // SHOULD FAIL
{
    let a: u64 = 100;
    let b: u64 = 200;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// Mutation Test 3: Assert u64 serialization produces wrong length (4 instead of 8)
// u64 serializes to exactly 8 bytes via spec_u64_to_le_bytes
proof fn test_mutation_wrong_serialization_length() // SHOULD FAIL
{
    let a: u64 = 0;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize().len() == 4);
}

}
