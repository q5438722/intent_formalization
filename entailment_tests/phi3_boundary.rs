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

// φ3: The is_marshalable precondition for usize allows the full u64 range.
// This is a weaker check: can we at least show there exist marshalable usize
// values equal to u64::MAX? (depends on arch word size)
// Actually let's just check: is_marshalable doesn't upper-bound below u64::MAX
proof fn phi3_usize_max_marshalable(x: usize)
    requires x as int == u64::MAX as int,
    ensures x.is_marshalable(),
{
    // is_marshalable requires *self as int <= u64::MAX, which holds
}

}
