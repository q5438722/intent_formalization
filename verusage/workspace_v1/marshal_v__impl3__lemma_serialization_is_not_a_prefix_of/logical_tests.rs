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

// ---- LOGICAL TESTS ----

// LOGICAL TEST 1: Assert serializations differ at a specific byte position
// The lemma only guarantees non-prefix, not that any particular byte differs
// spec_u64_to_le_bytes is opaque — cannot determine specific byte values
// SHOULD FAIL
proof fn test_logical_specific_byte_difference() {
    let a: u64 = 1u64;
    let b: u64 = 2u64;
    assert(a.ghost_serialize()[0] != b.ghost_serialize()[0]);
}

// LOGICAL TEST 2: Uniqueness fallacy — from a!=b and a!=c, try to derive b==c
// Uses Option None/Some for provable preconditions
// The lemma gives non-prefix for pairs, not equality between other pairs
// SHOULD FAIL
proof fn test_logical_uniqueness_fallacy() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    let c: Option<u64> = Some(1u64);
    // Both calls have valid preconditions: None vs Some, len(1) <= len(9+)
    a.lemma_serialization_is_not_a_prefix_of(&b);
    a.lemma_serialization_is_not_a_prefix_of(&c);
    // We know a.serialize is not a prefix of b or c
    // Try to derive b.serialize == c.serialize (invalid!)
    assert(b.ghost_serialize() =~= c.ghost_serialize());
}

// LOGICAL TEST 3: Assert tuple serialization is commutative
// (1,2) and (2,1) serialize as different concatenations — should not be equal
// SHOULD FAIL
proof fn test_logical_tuple_commutativity() {
    let t1: (u64, u64) = (1u64, 2u64);
    let t2: (u64, u64) = (2u64, 1u64);
    assert(t1.ghost_serialize() =~= t2.ghost_serialize());
}

// LOGICAL TEST 4: Assert the lemma conclusion without calling the lemma
// Tests if non-prefix is trivially provable from definitions alone
// SHOULD FAIL
proof fn test_logical_postcondition_without_lemma() {
    let a: u64 = 3u64;
    let b: u64 = 7u64;
    // Do NOT call the lemma — try to assert its postcondition directly
    assert(a.ghost_serialize() != b.ghost_serialize().subrange(0, a.ghost_serialize().len() as int));
}

}
