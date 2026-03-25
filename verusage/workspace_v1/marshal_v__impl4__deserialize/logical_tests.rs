use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ===== Original definitions (from target file) =====

#[verifier::external_body]
pub proof fn lemma_seq_add_subrange<A>(s: Seq<A>, i: int, j: int, k: int)
  requires 0 <= i <= j <= k <= s.len(),
  ensures s.subrange(i, j) + s.subrange(j, k) == s.subrange(i, k),
{
    unimplemented!()
}

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

// Test 1: Soundness — try to derive false after valid lemma invocation
// Valid use of a lemma should not allow proving false
// SHOULD FAIL
proof fn test_logical_derive_false() {
    let s = seq![1u8, 2u8, 3u8];
    lemma_seq_add_subrange::<u8>(s, 0, 1, 3);
    assert(false);
}

// Test 2: Assert u64 serialization length is 0
// u64 serializes to 8 bytes via spec_u64_to_le_bytes, not 0
// SHOULD FAIL
proof fn test_logical_u64_serialize_length_zero() {
    let x: u64 = 42;
    assert(x.ghost_serialize().len() == 0);
}

// Test 3: Assert None and Some(0) have equal serialization length
// None → seq![0] has length 1; Some(0u64) → seq![1] + 8 bytes has length 9
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

// Test 5: Assert two different u64 values produce the same serialization (non-injectivity)
// spec_u64_to_le_bytes is injective, so different values must produce different outputs
// SHOULD FAIL
proof fn test_logical_non_injective_serialization() {
    let x: u64 = 0;
    let y: u64 = 1;
    assert(x.ghost_serialize() =~= y.ghost_serialize());
}

}
