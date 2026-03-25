use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== Source definitions (from target file) ==========

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

// ========== LOGICAL TESTS ==========
// These tests assert properties NOT explicitly guaranteed by the specification.
// They should ALL FAIL verification.

// Test 1: Injectivity — assert that if serializations match, values must be view_equal
// The spec only guarantees: !view_equal → not-a-prefix (one direction).
// It does NOT guarantee: same serialization → view_equal (converse/injectivity).
// Without calling the lemma, Verus should not be able to derive this.
// SHOULD FAIL
proof fn test_logical_injectivity_without_lemma() {
    let a: u64 = 1;
    let b: u64 = 2;
    // Assert injectivity: if ghost_serialize matches, then view_equal
    // This is NOT guaranteed by the spec (would require a separate lemma)
    assume(a.ghost_serialize() =~= b.ghost_serialize());
    assert(a.view_equal(&b));
}

// Test 2: Prefix-freeness without calling the lemma
// The spec guarantees prefix-freeness only through the lemma.
// Without calling it, the conclusion should NOT be freely available.
// SHOULD FAIL
proof fn test_logical_prefix_free_without_lemma() {
    let a: u64 = 3;
    let b: u64 = 4;
    // Don't call the lemma — just assert the conclusion directly
    assert(a.ghost_serialize() != b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

// Test 3: Stronger pointwise property — assert ALL bytes differ
// The spec only says serialization is not a prefix (overall inequality).
// It does NOT say every individual byte position differs.
// SHOULD FAIL
proof fn test_logical_all_bytes_differ() {
    let a: u64 = 1;
    let b: u64 = 2;
    lemma_auto_spec_u64_to_from_le_bytes();
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // Stronger claim: assert every byte in the serialization is different
    // This is NOT guaranteed — e.g., 1u64 and 2u64 only differ in the first byte (LE)
    assert(forall |i: int| 0 <= i < a.ghost_serialize().len() ==>
        #[trigger] a.ghost_serialize()[i] != b.ghost_serialize()[i]);
}

// Test 4: Symmetry of the lemma — assert conclusion in the REVERSE direction
// without calling the lemma for the reversed pair.
// The spec provides prefix-freeness for (self, other) but not automatically (other, self).
// SHOULD FAIL
proof fn test_logical_reverse_without_lemma() {
    let a: u64 = 10;
    let b: u64 = 20;
    lemma_auto_spec_u64_to_from_le_bytes();
    // Only call lemma in one direction
    a.lemma_serialization_is_not_a_prefix_of(&b);
    // Assert the reverse direction WITHOUT calling b.lemma..(&a)
    // The spec does not freely provide this
    assert(b.ghost_serialize() != a.ghost_serialize().subrange(0, b.ghost_serialize().len() as int));
}

}
