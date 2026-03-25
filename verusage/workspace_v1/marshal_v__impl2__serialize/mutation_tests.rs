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
// BEHAVIORAL MUTATION TESTS — mutate expected outputs/relations
// ============================================================

// SHOULD FAIL: 0u64 and 1u64 are distinct values; their serializations must differ
proof fn mutation_different_u64_same_serialize() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// SHOULD FAIL: u64 serialization is 8 bytes (little-endian), not 4
proof fn mutation_u64_serialize_length_4() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 42u64;
    assert(v.ghost_serialize().len() == 4);
}

// SHOULD FAIL: Round-trip of 42 gives 42, not 99
proof fn mutation_u64_roundtrip_wrong_value() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 42u64;
    assert(spec_u64_from_le_bytes(v.ghost_serialize()) == 99u64);
}

// SHOULD FAIL: usize 42 and u64 42 produce identical serializations by definition
proof fn mutation_usize_u64_same_value_different_serialize() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let a: u64 = 42u64;
    let b: usize = 42usize;
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

// SHOULD FAIL: Vec<u8> ghost_serialize includes a length prefix;
// the total length is prefix_len + content_len, not just content_len.
// For content of 3 bytes, total = 8 + 3 = 11, not 3.
proof fn mutation_vec_u8_serialize_ignores_prefix() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let prefix = (3usize).ghost_serialize();
    let content = seq![1u8, 2u8, 3u8];
    let total = prefix + content;
    assert(total.len() == 3);
}

}
