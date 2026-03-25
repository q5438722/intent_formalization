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

// ---- BOUNDARY TESTS ----

// BOUNDARY TEST 1: u64 is always marshalable; asserting it is not should fail
// Targets: u64::is_marshalable() unconditionally returns true
// SHOULD FAIL
proof fn test_boundary_u64_not_marshalable() {
    let x: u64 = 42u64;
    assert(!x.is_marshalable());
}

// BOUNDARY TEST 2: None::<u64> is always marshalable; asserting it is not should fail
// Targets: Option::is_marshalable returns true for None regardless of T
// SHOULD FAIL
proof fn test_boundary_none_not_marshalable() {
    let x: Option<u64> = None;
    assert(!x.is_marshalable());
}

// BOUNDARY TEST 3: None::<u64> serializes to seq![0] (length 1), not empty
// Asserting length 0 should fail
// SHOULD FAIL
proof fn test_boundary_none_serialize_length_zero() {
    let x: Option<u64> = None;
    assert(x.ghost_serialize().len() == 0);
}

// BOUNDARY TEST 4: usize(0) satisfies is_marshalable since 0 <= u64::MAX
// Asserting it is not marshalable should fail
// SHOULD FAIL
proof fn test_boundary_usize_zero_not_marshalable() {
    let x: usize = 0usize;
    assert(!x.is_marshalable());
}

}
