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

// ==================== LOGICAL TESTS ====================
// These tests attempt to prove properties NOT entailed by the specification.
// Each SHOULD FAIL because the property is not derivable from the spec.

// Test 1: Converse - equal serialization implies view_equal
// SHOULD FAIL: the lemma proves view_equal → serialize equal, NOT the converse
proof fn test_logical_converse<T: Marshalable>(x: T, y: T)
    requires
        x.ghost_serialize() =~= y.ghost_serialize(),
{
    assert(x.view_equal(&y));
}

// Test 2: Symmetry of view_equal at trait level
// SHOULD FAIL: view_equal is not guaranteed to be symmetric by the trait spec
proof fn test_logical_symmetry<T: Marshalable>(x: T, y: T)
    requires
        x.view_equal(&y),
{
    assert(y.view_equal(&x));
}

// Test 3: Non-empty serialization for marshalable values
// SHOULD FAIL: the spec does not guarantee serialization produces any bytes
proof fn test_logical_nonempty_serialization<T: Marshalable>(x: T)
    requires
        x.is_marshalable(),
{
    assert(x.ghost_serialize().len() > 0);
}

// Test 4: Reflexivity of view_equal at trait level
// SHOULD FAIL: view_equal is not guaranteed to be reflexive by the trait spec alone
proof fn test_logical_reflexivity<T: Marshalable>(x: T) {
    assert(x.view_equal(&x));
}

// Test 5: Injectivity - different views imply different serializations
// SHOULD FAIL: the spec does not guarantee that non-view-equal values serialize differently
proof fn test_logical_injectivity<T: Marshalable>(x: T, y: T)
    requires
        !x.view_equal(&y),
        x.is_marshalable(),
        y.is_marshalable(),
{
    assert(!(x.ghost_serialize() =~= y.ghost_serialize()));
}

}
