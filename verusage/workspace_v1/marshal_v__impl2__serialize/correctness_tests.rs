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
// total length is 8 + 3 = 11, not 3.
proof fn mutation_vec_u8_serialize_ignores_prefix() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let prefix = (3usize).ghost_serialize();
    let content = seq![1u8, 2u8, 3u8];
    let total = prefix + content;
    assert(total.len() == 3);
}

// ============================================================
// LOGICAL TESTS — unintended properties / structural claims
// ============================================================

// SHOULD FAIL: u64 serialization is NOT constant; different values produce different bytes
proof fn logical_u64_serialize_constant() {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(forall |a: u64, b: u64| a.ghost_serialize() =~= b.ghost_serialize());
}

// SHOULD FAIL: No u64 can serialize to more than 8 bytes
proof fn logical_u64_serialize_unbounded() {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(exists |v: u64| v.ghost_serialize().len() > 8);
}

// SHOULD FAIL: u64 and usize with DIFFERENT values should NOT serialize identically
proof fn logical_cross_type_different_value_same_serialize() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let a: u64 = 0u64;
    let b: usize = 1usize;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// SHOULD FAIL: Vec<u8> ghost_serialize length is prefix_len + content_len,
// never just the content length. 8 + 5 = 13, not 5.
proof fn logical_vec_u8_serialize_length_equals_content() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let prefix = (5usize).ghost_serialize();
    let content = seq![10u8, 20u8, 30u8, 40u8, 50u8];
    let total = prefix + content;
    assert(total.len() == content.len());
}

// SHOULD FAIL: Asserting that u64 serialization of 1 is all-zero bytes
proof fn logical_u64_serialize_all_zeros() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 1u64;
    let s = v.ghost_serialize();
    assert(forall |i: int| 0 <= i < s.len() ==> s[i] == 0u8);
}

}
