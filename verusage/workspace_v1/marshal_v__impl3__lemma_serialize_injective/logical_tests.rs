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
// LOGICAL TESTS - Properties NOT explicitly guaranteed by the spec
// Each test SHOULD FAIL verification.
// ============================================================

// Test L1: Assert two distinct u64 values are view_equal
// The spec does NOT entail that distinct values are view_equal
// SHOULD FAIL
proof fn test_logical_false_view_equal_u64() {
    let a: u64 = 0;
    let b: u64 = 1;
    assert(a.view_equal(&b)); // false: 0@ !== 1@
}

// Test L2: Assert Option::None is view_equal to Option::Some
// Cross-variant view_equal is always false by definition, but
// this tests that the spec correctly distinguishes constructors
// SHOULD FAIL
proof fn test_logical_cross_variant_view_equal() {
    let a: Option<u64> = None;
    let b: Option<u64> = Some(0u64);
    assert(a.view_equal(&b)); // false by match arm: (None, Some) => false
}

// Test L3: Assert different u64 values have equal serializations
// spec_u64_to_le_bytes should be injective, but the spec doesn't
// explicitly guarantee this without calling the lemma
// SHOULD FAIL
proof fn test_logical_different_u64_same_serialization() {
    let a: u64 = 0;
    let b: u64 = 1;
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// Test L4: Assert view_equal for parametric types without calling lemma
// Tries to derive injectivity purely from structural reasoning
// without invoking lemma_serialize_injective
// SHOULD FAIL
proof fn test_logical_option_injectivity_without_lemma(a: Option<u64>, b: Option<u64>)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() =~= b.ghost_serialize(),
{
    // Attempt to conclude view_equal without calling lemma
    assert(a.view_equal(&b)); // should need the lemma to prove this
}

// Test L5: Assert tuple component serialization can be split
// without any knowledge of component lengths
// The tuple serialization is just concatenation, so splitting
// requires knowing where to split - not guaranteed without extra info
// SHOULD FAIL
proof fn test_logical_tuple_component_split(a: (u64, u64), b: (u64, u64))
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() =~= b.ghost_serialize(),
{
    // Try to assert first components have equal serialization
    // without calling lemma_serialize_injective on the tuple
    assert(a.0.ghost_serialize() =~= b.0.ghost_serialize());
}


}
