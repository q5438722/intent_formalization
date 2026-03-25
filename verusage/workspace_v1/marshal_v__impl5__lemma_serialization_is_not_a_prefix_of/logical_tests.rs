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
// LOGICAL TESTS — Properties NOT explicitly guaranteed
// ============================================================

// LT1: Stronger inequality — assert non-view-equal u64 values always differ
// at the FIRST byte of their serialization. This is stronger than "not a prefix".
// Counterexample: 256 and 512 both have LE first byte 0x00.
// SHOULD FAIL
proof fn test_lt1_first_byte_always_differs() {
    let x: u64 = 256;
    let y: u64 = 512;
    proof_fun_u64_len_helper();
    // Both serialize to LE with first byte 0:
    //   256 = [0x00, 0x01, 0, 0, 0, 0, 0, 0]
    //   512 = [0x00, 0x02, 0, 0, 0, 0, 0, 0]
    x.lemma_serialization_is_not_a_prefix_of(&y);
    // Try to assert a STRONGER property: first bytes differ
    assert(x.ghost_serialize()[0] != y.ghost_serialize()[0]);
}

// Helper to establish u64 serialization length = 8
proof fn proof_fun_u64_len_helper()
    ensures forall|x: u64| #[trigger] spec_u64_to_le_bytes(x).len() == 8,
{
    lemma_auto_spec_u64_to_from_le_bytes();
}

// LT2: Structural assumption — assert tuple serialization is commutative:
// (a, b).ghost_serialize() == (b, a).ghost_serialize()
// This is NOT true since concatenation is not commutative for different values.
// SHOULD FAIL
proof fn test_lt2_tuple_serialization_commutative() {
    let a: u64 = 1;
    let b: u64 = 2;
    let t1: (u64, u64) = (a, b);
    let t2: (u64, u64) = (b, a);
    // serialize(1) ++ serialize(2) vs serialize(2) ++ serialize(1)
    assert(t1.ghost_serialize() =~= t2.ghost_serialize());
}

// LT3: All-bytes-differ — assert non-view-equal u64 values differ at ALL 8 byte
// positions. The spec only guarantees they're not prefix-equal.
// Counterexample: 0 and 1 only differ at position 0.
// SHOULD FAIL
proof fn test_lt3_all_bytes_differ() {
    let x: u64 = 0;
    let y: u64 = 1;
    proof_fun_u64_len_helper();
    let sx = x.ghost_serialize();
    let sy = y.ghost_serialize();
    //   0 = [0, 0, 0, 0, 0, 0, 0, 0]
    //   1 = [1, 0, 0, 0, 0, 0, 0, 0]
    // They agree at positions 1..7
    assert(forall |i: int| 0 <= i < 8 ==> sx[i] != sy[i]);
}

// LT4: Reflexivity — the spec only guarantees symmetry of view_equal, NOT
// reflexivity. But for concrete u64, view_equal(x,x) is self@ === self@ which
// is trivially true. If this PASSES, it reveals the concrete implementation
// leaks reflexivity that the abstract spec does not guarantee.
// SHOULD FAIL (but may pass for concrete u64 — spec weakness finding)
proof fn test_lt4_view_equal_reflexivity_not_guaranteed() {
    let x: u64 = 99;
    assert(x.view_equal(&x));
}

// LT5: Cross-function misuse — try to derive that two different u64 values
// have the same serialization without using any lemma.
// The spec does not provide this; it should be rejected.
// SHOULD FAIL
proof fn test_lt5_same_length_implies_same_serialization() {
    let x: u64 = 100;
    let y: u64 = 200;
    proof_fun_u64_len_helper();
    // Both u64 values serialize to 8 bytes (verified via helper)
    assert(x.ghost_serialize().len() == y.ghost_serialize().len());
    // Now try to assert same content (should fail):
    assert(x.ghost_serialize() =~= y.ghost_serialize());
}

}
