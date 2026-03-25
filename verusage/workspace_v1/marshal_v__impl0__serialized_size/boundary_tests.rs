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

// ===== BOUNDARY TESTS =====

// FINDING: The following property PASSES verification, revealing a spec weakness.
// The is_marshalable guard for usize (*self as int <= u64::MAX) is vacuously true
// because Verus assumes usize fits in u64. The precondition provides no protection.
//
// proof fn finding_all_usize_marshalable() {
//     assert(forall|x: usize| (#[trigger] x.is_marshalable()));
// }

// Test 1: Assert u64 has some non-marshalable value.
// SHOULD FAIL: u64.is_marshalable() is unconditionally true by spec definition.
proof fn test_u64_has_non_marshalable_value() // SHOULD FAIL
{
    assert(exists|x: u64| !x.is_marshalable());
}

// Test 2: For a non-marshalable usize (exceeds u64::MAX), assert serialize length is 8.
// SHOULD FAIL: The recommends clause is violated, so we should not be
// able to reason about ghost_serialize for non-marshalable values.
proof fn test_non_marshalable_usize_serialize_len() // SHOULD FAIL
{
    let x: usize;
    assume(x as int > u64::MAX as int);
    // is_marshalable is false, ghost_serialize's recommends is violated
    assert(x.ghost_serialize().len() == 8);
}

// Test 3: Edge case u64::MAX — assert serialization has wrong length (4 instead of 8).
// SHOULD FAIL: serialization length for u64 is always 8, even at the boundary.
proof fn test_u64_max_wrong_serialize_length() // SHOULD FAIL
{
    lemma_auto_spec_u64_to_from_le_bytes();
    let x: u64 = u64::MAX;
    assert(x.ghost_serialize().len() == 4);
}

}
