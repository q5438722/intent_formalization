use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ===== Original definitions (from target file) =====

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  #[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
    ensures match res {
      Some((x, end)) => {
        &&& x.is_marshalable()
        &&& start <= end <= data.len()
        &&& data@.subrange(start as int, end as int) == x.ghost_serialize()
      }
      None => true,
    }
  {
    unimplemented!()
  }
}

impl Marshalable for u64 {
  open spec fn is_marshalable(&self) -> bool { true }
  open spec fn ghost_serialize(&self) -> Seq<u8> { spec_u64_to_le_bytes(*self) }
  #[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
  { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool { *self as int <= u64::MAX }
  open spec fn ghost_serialize(&self) -> Seq<u8> { (*self as u64).ghost_serialize() }
  #[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
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
  #[verifier::spinoff_prover]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
  { unimplemented!() }
}

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
  open spec fn is_marshalable(&self) -> bool {
    &&& self.0.is_marshalable()
    &&& self.1.is_marshalable()
    &&& self.0.ghost_serialize().len() + self.1.ghost_serialize().len() <= usize::MAX
  }
  open spec fn ghost_serialize(&self) -> Seq<u8> {
    self.0.ghost_serialize() + self.1.ghost_serialize()
  }
  #[verifier::external_body]
  #[verifier::spinoff_prover]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
  { unimplemented!() }
}

// ========== BOUNDARY TESTS ==========

// Test 1: Assert u64 value 0 has zero-length serialization
// u64 always serializes to 8 bytes via spec_u64_to_le_bytes
// SHOULD FAIL
proof fn test_boundary_u64_zero_serialize_length_zero() {
    let x: u64 = 0;
    assert(x.ghost_serialize().len() == 0);
}

// Test 2: Assert u64::MAX has serialization shorter than 8 bytes
// spec_u64_to_le_bytes always produces exactly 8 bytes regardless of value
// SHOULD FAIL
proof fn test_boundary_u64_max_serialize_short() {
    let x: u64 = u64::MAX;
    assert(x.ghost_serialize().len() < 8);
}

// Test 3: Assert Option::None is not marshalable
// None is always marshalable by definition
// SHOULD FAIL
proof fn test_boundary_option_none_not_marshalable() {
    let x: Option<u64> = Option::<u64>::None;
    assert(!x.is_marshalable());
}

// Test 4: Assert pair (0, 0) is not marshalable
// Both u64 are marshalable and 8+8=16 <= usize::MAX
// SHOULD FAIL
proof fn test_boundary_pair_not_marshalable() {
    let p: (u64, u64) = (0u64, 0u64);
    assert(!p.is_marshalable());
}

// Test 5: Assert Option::Some(0u64) serialization has length 1 (tag byte only)
// It should be 1 + 8 = 9 bytes (tag + u64 payload)
// SHOULD FAIL
proof fn test_boundary_option_some_serialize_tag_only() {
    let x: Option<u64> = Option::<u64>::Some(0u64);
    assert(x.ghost_serialize().len() == 1);
}

}
