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

impl Marshalable for usize {
  open spec fn view_equal(&self, other: &Self) -> bool { self@ === other@ }
  open spec fn is_marshalable(&self) -> bool { &&& *self as int <= u64::MAX }
  open spec fn ghost_serialize(&self) -> Seq<u8> { (*self as u64).ghost_serialize() }
  #[verifier::external_body]
  proof fn lemma_serialize_injective(self: &Self, other: &Self) { unimplemented!() }
}

// φ1: Cross-type collision — equal serialization across types implies equal views?
proof fn phi1_cross_type_collision(a: u64, b: usize)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
    ensures
        a@ == b@,
{
    // Can we derive this from the spec?
    // usize serializes as (*self as u64).ghost_serialize()
    // so if a.ghost_serialize() == b.ghost_serialize(),
    // that means spec_u64_to_le_bytes(a) == spec_u64_to_le_bytes(b as u64)
    // By u64 injectivity: a@ == (b as u64)@
    let b_as_u64 = b as u64;
    a.lemma_serialize_injective(&b_as_u64);
    // Now a@ == b_as_u64@ == b@
}

}
