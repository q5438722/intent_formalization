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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Assert Option::None serializes with wrong tag byte (1 instead of 0)
// None.ghost_serialize() == seq![0], not seq![1]
// SHOULD FAIL
proof fn test_mutation_option_none_wrong_tag() {
    let x: Option<u64> = Option::<u64>::None;
    assert(x.ghost_serialize() =~= seq![1u8]);
}

// Test 2: Assert Option::Some(42u64) serializes without tag prefix
// Some(v).ghost_serialize() == seq![1] + v.ghost_serialize(), not just v.ghost_serialize()
// SHOULD FAIL
proof fn test_mutation_option_some_missing_tag() {
    let v: u64 = 42;
    let x: Option<u64> = Option::<u64>::Some(v);
    assert(x.ghost_serialize() =~= v.ghost_serialize());
}

// Test 3: Assert pair serialization is in reversed component order
// (a,b).ghost_serialize() == a.gs() + b.gs(), NOT b.gs() + a.gs()
// SHOULD FAIL
proof fn test_mutation_pair_reversed_order() {
    let p: (u64, u64) = (1u64, 2u64);
    assert(p.ghost_serialize() =~= 2u64.ghost_serialize() + 1u64.ghost_serialize());
}

// Test 4: Assert two different u64 values produce the same serialization
// spec_u64_to_le_bytes(0) != spec_u64_to_le_bytes(1)
// SHOULD FAIL
proof fn test_mutation_u64_same_serialize_different_values() {
    assert(0u64.ghost_serialize() =~= 1u64.ghost_serialize());
}

// Test 5: Assert usize serialization differs from u64 for the same value
// usize::ghost_serialize delegates to (self as u64).ghost_serialize()
// So they must be equal; asserting inequality is wrong
// SHOULD FAIL
proof fn test_mutation_usize_differs_from_u64() {
    let x: usize = 42;
    let y: u64 = 42;
    assert(!(x.ghost_serialize() =~= y.ghost_serialize()));
}

}
