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

// === Boundary Tests ===

// Boundary Test 1: Call lemma with equal u64 values — violates !self.view_equal(other)
proof fn test_boundary_equal_u64_values() // SHOULD FAIL
{
    let a: u64 = 42;
    let b: u64 = 42;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Boundary Test 2: Call lemma with equal usize values — violates !self.view_equal(other)
proof fn test_boundary_equal_usize_values() // SHOULD FAIL
{
    let a: usize = 10;
    let b: usize = 10;
    a.lemma_serialization_is_not_a_prefix_of(&b);
}

// Boundary Test 3: Assert the postcondition for identical u64 values without calling lemma
// Since both values are equal, their serializations are identical, so != is false
proof fn test_boundary_postcondition_for_equal_values() // SHOULD FAIL
{
    let a: u64 = 7;
    let b: u64 = 7;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize() != b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

}
