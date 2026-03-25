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
// BOUNDARY TESTS - Violate preconditions (requires)
// Each test SHOULD FAIL verification.
// ============================================================

// Test B1: Call lemma on two different u64 values
// Violates precondition: ghost_serialize equality
// SHOULD FAIL
proof fn test_boundary_u64_different_values() {
    let a: u64 = 0;
    let b: u64 = 1;
    // 0u64 and 1u64 have different LE byte representations
    // so ghost_serialize(a) != ghost_serialize(b)
    a.lemma_serialize_injective(&b); // precondition violated
}

// Test B2: Call lemma on Option::None vs Option::Some
// Violates precondition: ghost_serialize equality
// SHOULD FAIL
proof fn test_boundary_option_none_vs_some() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    // None serializes as [0], Some(0) serializes as [1, 0, 0, 0, 0, 0, 0, 0, 0]
    a.lemma_serialize_injective(&b); // precondition violated
}

// Test B3: Call lemma on two different Option::Some values
// Violates precondition: ghost_serialize equality
// SHOULD FAIL
proof fn test_boundary_option_different_some() {
    let a: Option<u64> = Some(100u64);
    let b: Option<u64> = Some(200u64);
    // Different inner values produce different serializations
    a.lemma_serialize_injective(&b); // precondition violated
}

// Test B4: Call lemma on two different tuples
// Violates precondition: ghost_serialize equality
// SHOULD FAIL
proof fn test_boundary_tuple_different_values() {
    let a: (u64, u64) = (1u64, 2u64);
    let b: (u64, u64) = (3u64, 4u64);
    // Different component values produce different concatenated serializations
    a.lemma_serialize_injective(&b); // precondition violated
}

// Test B5: Call lemma on u64 max boundary values
// Violates precondition: ghost_serialize equality
// SHOULD FAIL
proof fn test_boundary_u64_zero_vs_max() {
    let a: u64 = 0u64;
    let b: u64 = u64::MAX;
    a.lemma_serialize_injective(&b); // precondition violated
}


}
