use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== DEFINITIONS FROM SOURCE ==========

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  { unimplemented!() }

  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>)
    requires self.is_marshalable()
    ensures
      data@.len() >= old(data).len(),
      data@.subrange(0, old(data)@.len() as int) == old(data)@,
      data@.subrange(old(data)@.len() as int, data@.len() as int) == self.ghost_serialize()
  { unimplemented!() }
}

impl Marshalable for u64 {
  open spec fn is_marshalable(&self) -> bool { true }
  open spec fn ghost_serialize(&self) -> Seq<u8> { spec_u64_to_le_bytes(*self) }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool { *self as int <= u64::MAX }
  open spec fn ghost_serialize(&self) -> Seq<u8> { (*self as u64).ghost_serialize() }
  #[verifier::external_body]
  exec fn serialize(&self, data: &mut Vec<u8>) { unimplemented!() }
}

// ========== LOGICAL TESTS ==========

// SHOULD FAIL: Assert fold_left over serialization is commutative (order-independent)
// Serialization concatenation is order-dependent; [1,2] and [2,1] produce different bytes
proof fn test_logical_fold_commutative() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let s1: Seq<u64> = seq![1u64, 2u64];
    let s2: Seq<u64> = seq![2u64, 1u64];
    assert(
        s1.fold_left(Seq::<u8>::empty(), |acc: Seq<u8>, x: u64| acc + spec_u64_to_le_bytes(x))
        =~=
        s2.fold_left(Seq::<u8>::empty(), |acc: Seq<u8>, x: u64| acc + spec_u64_to_le_bytes(x))
    );
}

// SHOULD FAIL: Assert fold_left of empty sequence with non-empty prefix returns empty
// fold_left base case: empty seq returns the initial accumulator (prefix), not Seq::empty()
proof fn test_logical_fold_empty_ignores_prefix() {
    let prefix: Seq<u8> = seq![1u8, 2u8, 3u8];
    let s: Seq<u64> = Seq::empty();
    assert(
        s.fold_left(prefix, |sb: Seq<u8>, a: u64| sb + spec_u64_to_le_bytes(a))
        =~=
        Seq::<u8>::empty()
    );
}

// SHOULD FAIL: Assert usize and u64 serialize differently for the same value
// usize::ghost_serialize explicitly delegates to u64, so they must be identical
proof fn test_logical_usize_u64_different_serialize() {
    lemma_auto_spec_u64_to_from_le_bytes();
    let u: usize = 42;
    let v: u64 = 42;
    assert(u.ghost_serialize() != v.ghost_serialize());
}

} // end verus!
