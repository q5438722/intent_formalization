use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;
use vstd::bytes::*;
use vstd::slice::*;

fn main() {}

verus!{

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;
  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}
  spec fn view_equal(&self, other: &Self) -> bool;
  #[verifier::external_body]
  proof fn lemma_serialize_injective(&self, other: &Self)
    requires self.is_marshalable(), other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
    ensures self.view_equal(other),
  {unimplemented!()}
}

impl Marshalable for u64 {
  open spec fn view_equal(&self, other: &Self) -> bool { self@ === other@ }
  open spec fn is_marshalable(&self) -> bool { true }
  open spec fn ghost_serialize(&self) -> Seq<u8> { spec_u64_to_le_bytes(*self) }
  proof fn lemma_serialize_injective(self: &Self, other: &Self) {
    lemma_auto_spec_u64_to_from_le_bytes();
  }
}

// φ4: If ghost_serialize returns empty, all values are view_equal
// Test: can we prove u64 serialization is never empty?
proof fn phi4_serialize_nonempty(a: u64)
    requires a.is_marshalable(),
    ensures a.ghost_serialize().len() > 0,
{
    lemma_auto_spec_u64_to_from_le_bytes();
    // spec_u64_to_le_bytes produces exactly 8 bytes
}

}
