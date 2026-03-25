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

// === LOGICAL TESTS ===
// Test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL: Not all u64 are non-marshalable; in fact, ALL are marshalable
proof fn logical_no_u64_is_marshalable() {
    assert(forall |v: u64| !v.is_marshalable());
}

// SHOULD FAIL: u64 and usize with the same value have identical serializations
// (usize delegates to u64), so asserting they differ should fail
proof fn logical_cross_type_different_serialize() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let a: u64 = 42u64;
    let b: usize = 42usize;
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

// SHOULD FAIL: No u64 value serializes to more than 8 bytes
proof fn logical_serialize_unbounded_length() {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(exists |v: u64| v.ghost_serialize().len() > 8);
}

}
