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

// ---- BEHAVIORAL MUTATION TESTS ----

// MUTATION TEST 1: None tag byte is 0; asserting it is 1 should fail
// Mutates the expected tag byte value for None variant
// SHOULD FAIL
proof fn test_mutation_none_wrong_tag() {
    let x: Option<u64> = None;
    assert(x.ghost_serialize()[0] == 1u8);
}

// MUTATION TEST 2: Some tag byte is 1; asserting it is 0 should fail
// Mutates the expected tag byte value for Some variant
// SHOULD FAIL
proof fn test_mutation_some_wrong_tag() {
    let x: Option<u64> = Some(0u64);
    assert(x.ghost_serialize()[0] == 0u8);
}

// MUTATION TEST 3: None serializes to seq![0]; asserting seq![1] should fail
// Mutates the entire serialized output of None
// SHOULD FAIL
proof fn test_mutation_none_wrong_serialized_value() {
    let x: Option<u64> = None;
    assert(x.ghost_serialize() =~= seq![1u8]);
}

// MUTATION TEST 4: None and Some(0u64) have different serializations
// Asserting they are equal should fail
// SHOULD FAIL
proof fn test_mutation_none_equals_some() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

}
