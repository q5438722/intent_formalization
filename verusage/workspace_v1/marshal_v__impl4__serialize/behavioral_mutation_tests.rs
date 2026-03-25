use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== DEFINITIONS FROM SOURCE ==========

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  { unimplemented!() }

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
    requires self.is_marshalable()
    ensures
      data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.len() as int) == self.ghost_serialize()
  { unimplemented!() }
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

// ========== BEHAVIORAL MUTATION TESTS ==========

// SHOULD FAIL: Mutate expected serialization length from 8 to 4
// u64 serialization always produces 8 bytes, not 4
proof fn test_mutation_u64_serialize_length_4() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: u64 = 0;
    assert(v.ghost_serialize().len() == 4);
}

// SHOULD FAIL: Assert two different u64 values produce identical serialization
// Round-trip property guarantees injectivity: different inputs -> different outputs
proof fn test_mutation_u64_noninjective() {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert((0u64).ghost_serialize() =~= (1u64).ghost_serialize());
}

// SHOULD FAIL: Assert usize 42 serializes to the same bytes as u64 43
// usize ghost_serialize delegates to (*self as u64), so 42usize -> 42u64, not 43u64
proof fn test_mutation_usize_wrong_delegate() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let v: usize = 42;
    assert(v.ghost_serialize() =~= (43u64).ghost_serialize());
}

} // end verus!
