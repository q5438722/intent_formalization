use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ---- Trait and impls (copied from source) ----

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
  open spec fn is_marshalable(&self) -> bool {
    true
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
  { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
  { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Option<T> {
  open spec fn is_marshalable(&self) -> bool {
    match self {
      None => true,
      Some(x) => x.is_marshalable() && 1 + x.ghost_serialize().len() <= usize::MAX,
    }
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    match self {
      None => seq![0],
      Some(x) => seq![1] + x.ghost_serialize(),
    }
  }

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
  { unimplemented!() }
}

// ---- LOGICAL TESTS ----

// LOGICAL TEST 1: Injectivity — different u64 values should not serialize identically
// The spec does not explicitly state injectivity, but spec_u64_to_le_bytes has an inverse
// Asserting equal serialization for distinct values should fail
// SHOULD FAIL
proof fn test_logical_u64_non_injective() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// LOGICAL TEST 2: Stronger length claim — Some(x) serialization should be longer than 1 byte
// Some(x) = seq![1] + x.ghost_serialize(), so len >= 1; asserting len == 1
// means x.ghost_serialize() is empty, which is not the case for u64 (8 bytes)
// SHOULD FAIL
proof fn test_logical_some_serialize_length_one() {
    let x: Option<u64> = Some(0u64);
    assert(x.ghost_serialize().len() == 1);
}

// LOGICAL TEST 3: None serialization is exactly 1 byte — asserting >= 2 should fail
// Tests whether spec allows unintended reasoning about serialization size
// SHOULD FAIL
proof fn test_logical_none_length_overestimate() {
    let x: Option<u64> = None;
    assert(x.ghost_serialize().len() >= 2);
}

// LOGICAL TEST 4: u64 serialization length — asserting it is 1 byte should fail
// spec_u64_to_le_bytes always returns 8 bytes; claiming 1 is wrong
// SHOULD FAIL
proof fn test_logical_u64_wrong_length() {
    let x: u64 = 0u64;
    assert(x.ghost_serialize().len() == 1);
}

}
