use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ===== Source definitions (from marshal_v__impl0__serialized_size.rs) =====

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  { unimplemented!() }

  #[verifier::external_body]
  exec fn serialized_size(&self) -> (res: usize)
    requires self.is_marshalable(),
    ensures res as int == self.ghost_serialize().len()
  { unimplemented!() }
}

impl Marshalable for u64 {
  open spec fn is_marshalable(&self) -> bool { true }
  open spec fn ghost_serialize(&self) -> Seq<u8> { spec_u64_to_le_bytes(*self) }

  #[verifier::spinoff_prover]
  exec fn serialized_size(&self) -> (res: usize) {
    proof { lemma_auto_spec_u64_to_from_le_bytes(); }
    8
  }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool { *self as int <= u64::MAX as int }
  open spec fn ghost_serialize(&self) -> Seq<u8> { (*self as u64).ghost_serialize() }

  #[verifier::external_body]
  exec fn serialized_size(&self) -> (res: usize) { unimplemented!() }
}

// ===== LOGICAL TESTS =====

// Test 1: Assert injectivity of ghost_serialize for u64.
// SHOULD FAIL: Injectivity is not explicitly guaranteed by the spec;
// the ensures clause only states serialized_size == ghost_serialize().len(),
// it does not assert that ghost_serialize is injective.
proof fn test_u64_serialize_injective() // SHOULD FAIL
{
    assert(forall|a: u64, b: u64|
        a.ghost_serialize() =~= b.ghost_serialize() ==> a == b);
}

// Test 2: Assert every u64's first serialized byte is zero.
// SHOULD FAIL: This is a stronger property not stated in the spec;
// only values whose low byte is 0 would satisfy this.
proof fn test_u64_first_byte_always_zero() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(forall|x: u64| (#[trigger] x.ghost_serialize())[0] == 0u8);
}

// Test 3: Assert last byte of serialization is always zero for u64.
// SHOULD FAIL: For values >= 2^56, the high byte is nonzero;
// this property is not guaranteed by the spec.
proof fn test_u64_last_byte_always_zero() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(forall|x: u64| (#[trigger] x.ghost_serialize())[7] == 0u8);
}

}
