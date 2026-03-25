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

// ==================== BOUNDARY TESTS ====================
// These tests violate preconditions (requires clauses).
// Each SHOULD FAIL because the precondition `self.view_equal(other)` is not satisfied.

// Test 1: Non-view-equal u64 values (0 vs 1)
// SHOULD FAIL: view_equal requires self@ === other@, but 0 != 1
proof fn test_boundary_u64_different_values() {
    let x: u64 = 0;
    let y: u64 = 1;
    x.lemma_same_views_serialize_the_same(&y);
}

// Test 2: Option Some vs None
// SHOULD FAIL: view_equal(Some(_), None) is false by definition
proof fn test_boundary_option_some_vs_none() {
    let x: Option<u64> = Some(0u64);
    let y: Option<u64> = None;
    x.lemma_same_views_serialize_the_same(&y);
}

// Test 3: Options with different inner values
// SHOULD FAIL: view_equal(Some(0), Some(1)) is false because 0 != 1
proof fn test_boundary_option_different_inner() {
    let x: Option<u64> = Some(0u64);
    let y: Option<u64> = Some(1u64);
    x.lemma_same_views_serialize_the_same(&y);
}

// Test 4: Tuples differing in first component
// SHOULD FAIL: view_equal requires both components to be view_equal
proof fn test_boundary_tuple_different_first() {
    let x: (u64, u64) = (0u64, 5u64);
    let y: (u64, u64) = (1u64, 5u64);
    x.lemma_same_views_serialize_the_same(&y);
}

// Test 5: Tuples differing in second component
// SHOULD FAIL: view_equal requires both components to be view_equal
proof fn test_boundary_tuple_different_second() {
    let x: (u64, u64) = (5u64, 0u64);
    let y: (u64, u64) = (5u64, 1u64);
    x.lemma_same_views_serialize_the_same(&y);
}

}
