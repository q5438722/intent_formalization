use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// ========== Trait and Impl Definitions (from source) ==========

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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Negate the postcondition — assert NOT view_equal after valid lemma call
// SHOULD FAIL
proof fn test_mutation_negate_postcondition(a: u64, b: u64)
  requires
    a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    // Lemma ensures a.view_equal(&b), so the negation should fail
    assert(!a.view_equal(&b));
}

// Test 2: Assert wrong inequality relation (a > b) after valid lemma call
// SHOULD FAIL
proof fn test_mutation_wrong_inequality(a: u64, b: u64)
  requires
    a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    // Lemma ensures a@ === b@, not a@ > b@
    assert(a@ > b@);
}

// Test 3: Assert off-by-one relationship after valid lemma call
// SHOULD FAIL
proof fn test_mutation_off_by_one(a: u64, b: u64)
  requires
    a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    // Lemma ensures a@ === b@, not a@ == b@ + 1
    assert(a@ == b@ + 1);
}

// Test 4: Assert serializations differ after lemma proves view_equal
// SHOULD FAIL
proof fn test_mutation_assert_serialize_differ(a: u64, b: u64)
  requires
    a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    // Precondition already says serializations are equal; asserting they differ is contradictory
    assert(a.ghost_serialize() !== b.ghost_serialize());
}

}
