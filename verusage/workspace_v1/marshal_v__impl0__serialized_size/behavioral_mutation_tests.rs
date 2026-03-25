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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Mutate expected serialized_size from 8 to 4 for u64.
// SHOULD FAIL: The correct serialized size for u64 is always 8.
proof fn test_u64_serialize_length_mutated_to_4() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    let x: u64 = 42;
    assert(x.ghost_serialize().len() == 4);
}

// Test 2: Assert two distinct u64 values produce identical serializations.
// SHOULD FAIL: Different u64 values should serialize to different byte sequences.
proof fn test_distinct_u64_same_serialization() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(0u64.ghost_serialize() =~= 1u64.ghost_serialize());
}

// Test 3: Assert ghost_serialize for u64 produces an empty sequence.
// SHOULD FAIL: u64 always serializes to exactly 8 bytes, never empty.
proof fn test_u64_serialize_empty() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    let x: u64 = 0;
    assert(x.ghost_serialize() =~= Seq::<u8>::empty());
}

}
