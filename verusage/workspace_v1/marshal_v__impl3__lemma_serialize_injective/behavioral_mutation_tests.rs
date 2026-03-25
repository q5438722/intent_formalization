use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ---- Trait and impl definitions (from source) ----

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable(),
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_serialize_injective(&self, other: &Self)
    requires
      self.is_marshalable(),
      other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
    ensures
      self.view_equal(other),
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
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
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
  proof fn lemma_serialize_injective(self: &Self, other: &Self)
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

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    match self {
      None => seq![0],
      Some(x) => seq![1] + x.ghost_serialize(),
    }
  }

  #[verifier::spinoff_prover]
  proof fn lemma_serialize_injective(self: &Self, other: &Self) {
    match (self, other) {
      (Some(s), Some(o)) => {
        assert(s.ghost_serialize() =~= self.ghost_serialize().subrange(1, self.ghost_serialize().len() as int));
        assert(o.ghost_serialize() =~= other.ghost_serialize().subrange(1, other.ghost_serialize().len() as int));
        s.lemma_serialize_injective(o);
      },
      (None, None) => {},
      (Some(s), None) => {
        assert(other.ghost_serialize()[0] == 0); // OBSERVE
      },
      (None, Some(o)) => {
        assert(self.ghost_serialize()[0] == 0); // OBSERVE
      }
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
  proof fn lemma_serialize_injective(self: &Self, other: &Self) {
    unimplemented!()
  }
}


// ============================================================
// BEHAVIORAL MUTATION TESTS - Mutate expected outputs/relations
// Each test SHOULD FAIL verification.
// ============================================================

// Test M1: Negate the ensures clause for u64
// After calling lemma with valid inputs, assert view_equal is FALSE
// SHOULD FAIL
proof fn test_mutation_negate_u64_view_equal() {
    let a: u64 = 42;
    let b: u64 = 42;
    a.lemma_serialize_injective(&b);
    assert(!a.view_equal(&b)); // negation of ensures
}

// Test M2: Negate the ensures clause for Option::None
// After calling lemma with valid inputs, assert view_equal is FALSE
// SHOULD FAIL
proof fn test_mutation_negate_option_view_equal() {
    let a: Option<u64> = None;
    let b: Option<u64> = None;
    a.lemma_serialize_injective(&b);
    assert(!a.view_equal(&b)); // negation of ensures
}

// Test M3: Assert wrong serialization output for Option::None
// None serializes as [0], mutate to claim it is [1]
// SHOULD FAIL
proof fn test_mutation_wrong_none_serialization() {
    let a: Option<u64> = None;
    assert(a.ghost_serialize() =~= seq![1u8]); // wrong: should be seq![0]
}

// Test M4: Assert wrong serialization tag for Option::Some
// Some(x) serializes with tag byte 1, mutate to claim tag is 0
// SHOULD FAIL
proof fn test_mutation_wrong_some_tag() {
    let a: Option<u64> = Some(0u64);
    assert(a.ghost_serialize()[0] == 0u8); // wrong: Some tag is 1, not 0
}

// Test M5: Assert that two u64 values with same serialization have DIFFERENT views
// Directly mutates the relationship: same serialize → NOT view_equal
// SHOULD FAIL
proof fn test_mutation_u64_same_serialize_not_equal() {
    let a: u64 = 99;
    let b: u64 = 99;
    a.lemma_serialize_injective(&b);
    // lemma ensures a.view_equal(&b), i.e., a@ === b@
    // mutate: claim they are not equal
    assert(a@ !== b@);
}


}
