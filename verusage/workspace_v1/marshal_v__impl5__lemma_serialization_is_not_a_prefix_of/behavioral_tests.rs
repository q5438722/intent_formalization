use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// === Trait and impl definitions (from source) ===

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
    ensures self.view_equal(other) == other.view_equal(self)
  {unimplemented!()}

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
    requires
      !self.view_equal(other),
      self.ghost_serialize().len() <= other.ghost_serialize().len(),
    ensures
      self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)
  {unimplemented!()}

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
    requires
      self.view_equal(other),
    ensures
      self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize()
  {unimplemented!()}
}

impl Marshalable for u64 {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
  { unimplemented!() }

  open spec fn is_marshalable(&self) -> bool {
    true
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  { unimplemented!() }

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self@ === other@
  }

  #[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
  { unimplemented!() }

  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

  #[verifier::external_body]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  { unimplemented!() }

  #[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  { unimplemented!() }
}

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
  open spec fn view_equal(&self, other: &Self) -> bool {
    self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
  }

  #[verifier::external_body]
  #[verifier::spinoff_prover]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
  { unimplemented!() }

  open spec fn is_marshalable(&self) -> bool {
    &&& self.0.is_marshalable()
    &&& self.1.is_marshalable()
    &&& self.0.ghost_serialize().len() + self.1.ghost_serialize().len() <= usize::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    self.0.ghost_serialize() + self.1.ghost_serialize()
  }

  #[verifier::spinoff_prover]
  proof fn lemma_serialization_is_not_a_prefix_of(&self, other: &Self)
  {
    let si = self.ghost_serialize();
    let so = other.ghost_serialize();
    let mid: int = 0;
    if !self.0.view_equal(&other.0) {
      let (x0, x1) = (self.0, other.0);
      let (s0, s1) = (x0.ghost_serialize(), x1.ghost_serialize());
      x0.lemma_view_equal_symmetric(&x1);
      let (x0, x1, s0, s1) = if s0.len() <= s1.len() {
        (x0, x1, s0, s1)
      } else {
        (x1, x0, s1, s0)
      };
      x0.lemma_serialization_is_not_a_prefix_of(&x1);
      assert(!(s0 =~= s1.subrange(0, s0.len() as int)));
      let idx = choose |i:int| 0 <= i < s0.len() as int && s0[i] != s1[i];
      if si == so.subrange(0, si.len() as int) {
        assert(si[mid + idx] == so[mid + idx]);
      }
      return;
    } else {
      self.0.lemma_same_views_serialize_the_same(&other.0);
    }
    let mid = mid + self.0.ghost_serialize().len();
    if !self.1.view_equal(&other.1) {
      let (x0, x1) = (self.1, other.1);
      let (s0, s1) = (x0.ghost_serialize(), x1.ghost_serialize());
      x0.lemma_view_equal_symmetric(&x1);
      let (x0, x1, s0, s1) = if s0.len() <= s1.len() {
        (x0, x1, s0, s1)
      } else {
        (x1, x0, s1, s0)
      };
      x0.lemma_serialization_is_not_a_prefix_of(&x1);
      assert(!(s0 =~= s1.subrange(0, s0.len() as int)));
      let idx = choose |i:int| 0 <= i < s0.len() as int && s0[i] != s1[i];
      if si == so.subrange(0, si.len() as int) {
        assert(si[mid + idx] == so[mid + idx]);
      }
      return;
    } else {
      self.1.lemma_same_views_serialize_the_same(&other.1);
    }
  }

  #[verifier::external_body]
  #[verifier::spinoff_prover]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
  { unimplemented!() }
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — Valid inputs, mutated outputs
// ============================================================

// MT1: After calling lemma_serialization_is_not_a_prefix_of with valid inputs,
// assert the OPPOSITE of the postcondition: serialization IS a prefix.
// SHOULD FAIL
proof fn test_mt1_assert_serialization_is_prefix() {
    let x: u64 = 5;
    let y: u64 = 10;
    proof_fun_u64_len_helper();
    x.lemma_serialization_is_not_a_prefix_of(&y);
    // Postcondition gives us: x.ghost_serialize() != y.ghost_serialize().subrange(0, len)
    // Assert the opposite (mutated output):
    assert(x.ghost_serialize() =~= y.ghost_serialize().subrange(0, x.ghost_serialize().len() as int));
}

// Helper to establish u64 serialization length = 8
proof fn proof_fun_u64_len_helper()
    ensures forall|x: u64| #[trigger] spec_u64_to_le_bytes(x).len() == 8,
{
    lemma_auto_spec_u64_to_from_le_bytes();
}

// MT2: After calling lemma_same_views_serialize_the_same with valid inputs,
// assert that serializations are DIFFERENT (mutated output).
// SHOULD FAIL
proof fn test_mt2_assert_same_views_different_serialization() {
    let x: u64 = 42;
    let y: u64 = 42;
    x.lemma_same_views_serialize_the_same(&y);
    // Postcondition gives us: x.ghost_serialize() == y.ghost_serialize()
    // Assert the opposite (mutated output):
    assert(x.ghost_serialize() !== y.ghost_serialize());
}

// MT3: After calling lemma_view_equal_symmetric, assert that view_equal
// is NOT symmetric (mutated relation).
// SHOULD FAIL
proof fn test_mt3_assert_view_equal_not_symmetric() {
    let x: u64 = 5;
    let y: u64 = 10;
    x.lemma_view_equal_symmetric(&y);
    // Postcondition gives us: x.view_equal(&y) == y.view_equal(&x)
    // Assert the opposite (mutated relation):
    assert(x.view_equal(&y) != y.view_equal(&x));
}

// MT4: After calling lemma_same_views_serialize_the_same on a tuple,
// assert that is_marshalable differs (mutated marshalability relation).
// SHOULD FAIL
proof fn test_mt4_assert_same_views_different_marshalability() {
    let x: (u64, u64) = (3u64, 4u64);
    let y: (u64, u64) = (3u64, 4u64);
    x.lemma_same_views_serialize_the_same(&y);
    // Postcondition gives us: x.is_marshalable() == y.is_marshalable()
    // Assert the opposite (mutated output):
    assert(x.is_marshalable() != y.is_marshalable());
}

// MT5: After calling lemma_serialization_is_not_a_prefix_of on tuples,
// assert the serialization IS a prefix (mutated output on tuples).
// SHOULD FAIL
proof fn test_mt5_assert_tuple_serialization_is_prefix() {
    let x: (u64, u64) = (1u64, 2u64);
    let y: (u64, u64) = (3u64, 4u64);
    proof_fun_u64_len_helper();
    x.lemma_serialization_is_not_a_prefix_of(&y);
    // Assert the opposite of the postcondition:
    assert(x.ghost_serialize() =~= y.ghost_serialize().subrange(0, x.ghost_serialize().len() as int));
}

}
