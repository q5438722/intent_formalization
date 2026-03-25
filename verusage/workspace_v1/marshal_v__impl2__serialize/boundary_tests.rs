use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;
use vstd::bytes::*;
use vstd::slice::*;

fn main() {}

verus! {

// === Source definitions (spec-only, exec fns as external_body) ===

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
    requires self.is_marshalable()
    ensures
      data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.len() as int) == self.ghost_serialize(),
  {unimplemented!()}
}

impl Marshalable for u64 {
  open spec fn is_marshalable(&self) -> bool { true }
  open spec fn ghost_serialize(&self) -> Seq<u8> { spec_u64_to_le_bytes(*self) }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool { *self as int <= u64::MAX }
  open spec fn ghost_serialize(&self) -> Seq<u8> { (*self as u64).ghost_serialize() }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

impl Marshalable for Vec<u8> {
  open spec fn is_marshalable(&self) -> bool {
    self@.len() <= usize::MAX &&
    (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
  }
  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (self@.len() as usize).ghost_serialize() + self@
  }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Vec<T> {
  open spec fn is_marshalable(&self) -> bool {
    &&& self@.len() <= usize::MAX
    &&& (forall |x: T| self@.contains(x) ==> #[trigger] x.is_marshalable())
    &&& (self@.len() as usize).ghost_serialize().len() +
        self@.fold_left(0, |acc: int, x: T| acc + x.ghost_serialize().len()) <= usize::MAX
  }
  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (self@.len() as usize).ghost_serialize()
      + self@.fold_left(Seq::<u8>::empty(), |acc: Seq<u8>, x: T| acc + x.ghost_serialize())
  }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

// ============================================================
// BOUNDARY TESTS — violate preconditions / use edge cases
// ============================================================

// SHOULD FAIL: u64::MAX is always marshalable (all u64 are)
proof fn boundary_u64_max_not_marshalable() {
    let v: u64 = u64::MAX;
    assert(!v.is_marshalable());
}

// SHOULD FAIL: usize 0 serialization is 8 bytes, not 0
proof fn boundary_usize_zero_serialize_empty() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: usize = 0usize;
    assert(v.ghost_serialize().len() == 0);
}

// SHOULD FAIL: u64 serialization is always 8 bytes, never exceeds it
proof fn boundary_u64_serialize_length_exceeds_8() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 0u64;
    assert(v.ghost_serialize().len() > 8);
}

// SHOULD FAIL: u64 serialization is always 8 bytes, never less
proof fn boundary_u64_serialize_length_under_8() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 1u64;
    assert(v.ghost_serialize().len() < 8);
}

// SHOULD FAIL: usize 0 is marshalable (0 <= u64::MAX)
proof fn boundary_usize_zero_not_marshalable() {
    let v: usize = 0usize;
    assert(!v.is_marshalable());
}

}
