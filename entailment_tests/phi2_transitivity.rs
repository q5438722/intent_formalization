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

// φ2: view_equal transitivity at trait level
// For u64, view_equal is ===, so transitivity holds trivially.
// We test if Verus can prove it from the spec alone.
proof fn phi2_view_equal_transitive(a: u64, b: u64, c: u64)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        c.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
        b.ghost_serialize() == c.ghost_serialize(),
    ensures
        a.view_equal(&c),
{
    a.lemma_serialize_injective(&b);
    b.lemma_serialize_injective(&c);
    // a@ === b@ and b@ === c@, so a@ === c@
}

}
