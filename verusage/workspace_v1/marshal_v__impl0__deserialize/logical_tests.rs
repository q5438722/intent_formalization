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

// ========== LOGICAL TESTS ==========

// Test 1: Try to derive false from valid spec operations
// No valid reasoning chain should yield false
// SHOULD FAIL
proof fn test_logical_derive_false() {
    let x: u64 = 0;
    let _ = x.ghost_serialize();
    assert(false);
}

// Test 2: Assert u64 serialization length varies by value
// spec_u64_to_le_bytes always produces 8 bytes regardless of the value
// SHOULD FAIL
proof fn test_logical_u64_serialize_length_varies() {
    let a: u64 = 0;
    let b: u64 = 1;
    assert(a.ghost_serialize().len() != b.ghost_serialize().len());
}

// Test 3: Assert None and Some(0u64) have equal serialization length
// None serializes to 1 byte (seq![0]); Some(0u64) serializes to 9 bytes (seq![1] + 8 bytes)
// SHOULD FAIL
proof fn test_logical_none_some_same_length() {
    let none: Option<u64> = Option::<u64>::None;
    let some: Option<u64> = Option::<u64>::Some(0u64);
    assert(none.ghost_serialize().len() == some.ghost_serialize().len());
}

// Test 4: Assert pair serialization length equals only the first component's length
// (a,b).gs().len() == a.gs().len() + b.gs().len(), not just a.gs().len()
// SHOULD FAIL
proof fn test_logical_pair_length_first_only() {
    let p: (u64, u64) = (1u64, 2u64);
    assert(p.ghost_serialize().len() == 1u64.ghost_serialize().len());
}

// Test 5: Assert commutativity of pair serialization
// (1,2).gs() = 1.gs() + 2.gs() which is NOT the same as (2,1).gs() = 2.gs() + 1.gs()
// SHOULD FAIL
proof fn test_logical_pair_serialize_commutative() {
    let p1: (u64, u64) = (1u64, 2u64);
    let p2: (u64, u64) = (2u64, 1u64);
    assert(p1.ghost_serialize() =~= p2.ghost_serialize());
}

}
