use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ==================== SOURCE DEFINITIONS ====================

pub trait Marshalable : Sized {

  spec fn is_marshalable(&self) -> bool;

	#[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
    requires
      self.view_equal(other),
    ensures
      self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
  {unimplemented!()}

}


impl Marshalable for u64 {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  open spec fn is_marshalable(&self) -> bool {
    true
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}


impl Marshalable for usize {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}


impl<T: Marshalable> Marshalable for Option<T> {

  open spec fn view_equal(&self, other: &Self) -> bool {
    match (self, other) {
      (None, None) => true,
      (Some(s), Some(o)) => s.view_equal(o),
      _ => false,
    }
  }

  open spec fn is_marshalable(&self) -> bool {
    match self {
      None => true,
      Some(x) => x.is_marshalable() && 1 + x.ghost_serialize().len() <= usize::MAX,
    }
  }

  open spec fn ghost_serialize(&self) -> Seq<u8>
  {
    match self {
      None => seq![0],
      Some(x) => seq![1] + x.ghost_serialize(),
    }
  }

  #[verifier::spinoff_prover]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  {
    match (self, other) {
      (Some(s), Some(o)) => s.lemma_same_views_serialize_the_same(o),
      _ => (),
    }
  }

}


impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
  }

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
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}

// ==================== BEHAVIORAL MUTATION TESTS ====================
// These tests start from valid inputs but assert mutated/incorrect outputs.
// Each SHOULD FAIL because the asserted behavior contradicts the specification.

// Test 1: Assert serializations are NOT equal for view-equal u64 values
// SHOULD FAIL: lemma ensures ghost_serialize equality for view-equal values
proof fn test_mutation_serialize_not_equal() {
    let x: u64 = 42;
    let y: u64 = 42;
    x.lemma_same_views_serialize_the_same(&y);
    assert(x.ghost_serialize() !== y.ghost_serialize());
}

// Test 2: Assert is_marshalable differs for view-equal values
// SHOULD FAIL: lemma ensures is_marshalable equality for view-equal values
proof fn test_mutation_marshalable_differs() {
    let x: u64 = 42;
    let y: u64 = 42;
    x.lemma_same_views_serialize_the_same(&y);
    assert(x.is_marshalable() != y.is_marshalable());
}

// Test 3: Assert Option::None serializes with wrong tag byte (1 instead of 0)
// SHOULD FAIL: None serializes to seq![0], not seq![1]
proof fn test_mutation_none_wrong_tag() {
    let x: Option<u64> = None;
    assert(x.ghost_serialize() =~= seq![1u8]);
}

// Test 4: Assert u64 is NOT marshalable (mutated marshalability)
// SHOULD FAIL: u64 is always marshalable (is_marshalable returns true)
proof fn test_mutation_u64_not_marshalable() {
    let x: u64 = 0;
    assert(!x.is_marshalable());
}

// Test 5: Assert Option::Some serializes with wrong tag byte (0 instead of 1)
// SHOULD FAIL: Some(x) serializes to seq![1] + x.ghost_serialize()
proof fn test_mutation_some_wrong_tag() {
    let x: u64 = 0;
    let opt: Option<u64> = Some(x);
    assert(opt.ghost_serialize()[0] == 0u8);
}

}
