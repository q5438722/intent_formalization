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

// === Logical Tests ===

// Logical Test 1: Assert stronger byte-level property — different values differ at byte 0
// Counterexample: 1u64 LE=[1,0,0,0,0,0,0,0], 257u64 LE=[1,1,0,0,0,0,0,0] — same byte 0
// The spec only guarantees non-prefix, not per-byte distinctness
proof fn test_logical_first_byte_always_differs() // SHOULD FAIL
{
    let a: u64 = 1;
    let b: u64 = 257;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize().index(0) != b.ghost_serialize().index(0));
}

// Logical Test 2: Assert serialization concatenation is commutative
// serialize(a) ++ serialize(b) == serialize(b) ++ serialize(a) is false for a != b
// The spec says nothing about concatenation order; this is an unwarranted structural assumption
proof fn test_logical_serialization_concat_commutative() // SHOULD FAIL
{
    let a: u64 = 1;
    let b: u64 = 2;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize() + b.ghost_serialize() =~= b.ghost_serialize() + a.ghost_serialize());
}

// Logical Test 3: Assert different values always differ at the last byte (byte 7)
// Counterexample: 1u64 LE=[1,0,...,0], 2u64 LE=[2,0,...,0] — both have byte 7 = 0
// The spec guarantees non-prefix (i.e., overall inequality) but not which byte differs
proof fn test_logical_last_byte_always_differs() // SHOULD FAIL
{
    let a: u64 = 1;
    let b: u64 = 2;
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize().index(7) != b.ghost_serialize().index(7));
}

}
