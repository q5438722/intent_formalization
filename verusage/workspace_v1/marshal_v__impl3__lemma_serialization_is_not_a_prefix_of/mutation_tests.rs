use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ---- Trait and impls (copied from source) ----

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable(),
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
    requires
      !self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),
    ensures
      self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int),
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self) {
    match (self, other) {
      (None, None) => {}
      (Some(_), None) | (None, Some(_)) => {
        assert(self.ghost_serialize()[0] != other.ghost_serialize()[0]);
      }
      (Some(s), Some(o)) => {
        s.lemma_serialization_is_not_a_prefix_of(o);
        assert(s.ghost_serialize() =~= self.ghost_serialize().subrange(1, self.ghost_serialize().len() as int));
        assert(o.ghost_serialize().subrange(0, s.ghost_serialize().len() as int) =~= other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int).subrange(1, self.ghost_serialize().len() as int));
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
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  {
    unimplemented!()
  }
}

// ---- BEHAVIORAL MUTATION TESTS ----

// MUTATION TEST 1: Assert prefix equality after lemma establishes non-prefix
// Uses Option None/Some where preconditions are provable from open definitions
// Directly contradicts the ensures clause
// SHOULD FAIL
proof fn test_mutation_assert_prefix() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    // Preconditions: !view_equal (None vs Some) ✓, len(1) <= len(9+) ✓
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // Postcondition: a.serialize() != b.serialize().subrange(0, a.serialize().len())
    // Assert the opposite (mutated):
    assert(a.ghost_serialize() =~= b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// MUTATION TEST 2: Assert None and Some have equal tag byte
// None serializes as [0], Some serializes as [1, ...]; first bytes differ
// SHOULD FAIL
proof fn test_mutation_wrong_tag_byte() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    // a.ghost_serialize() = seq![0], b.ghost_serialize() = seq![1] + ...
    assert(a.ghost_serialize()[0] == b.ghost_serialize()[0]);
}

// MUTATION TEST 3: Assert Some serializes with tag byte 0 (wrong — actual tag is 1)
// Tests if spec correctly rejects mutated serialization format
// SHOULD FAIL
proof fn test_mutation_wrong_some_tag() {
    let a: Option<u64> = Some(0u64);
    // Some serializes as seq![1] + x.ghost_serialize(), first byte is 1
    assert(a.ghost_serialize()[0] == 0u8);
}

}
