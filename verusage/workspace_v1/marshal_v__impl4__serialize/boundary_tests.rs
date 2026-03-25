use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== DEFINITIONS FROM SOURCE ==========

#[verifier::external_body]
pub proof fn lemma_seq_fold_left_append_right<A, B>(s: Seq<A>, prefix: Seq<B>, f: spec_fn(A) -> Seq<B>)
  requires s.len() > 0,
  ensures
    s.subrange(0, s.len() - 1).fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a)) + f(s[s.len() - 1])
    ==
    s.fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a))
{ unimplemented!() }

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

// ========== BOUNDARY TESTS ==========

// SHOULD FAIL: Violates requires clause (s.len() > 0) of lemma_seq_fold_left_append_right
// Boundary: empty sequence is below the minimum required input size
proof fn test_boundary_empty_seq_precondition() {
    let s: Seq<u64> = Seq::empty();
    let prefix: Seq<u8> = Seq::empty();
    lemma_seq_fold_left_append_right::<u64, u8>(s, prefix, |x: u64| spec_u64_to_le_bytes(x));
}

// SHOULD FAIL: Assert 0usize is NOT marshalable (edge case: minimum value)
// Boundary: 0 is well within u64::MAX, so is_marshalable should be true
proof fn test_boundary_zero_usize_not_marshalable() {
    assert(!(0usize).is_marshalable());
}

// SHOULD FAIL: Assert u64::MAX serialization has wrong length (edge case: maximum value)
// Boundary: even at max value, spec_u64_to_le_bytes always produces 8 bytes
proof fn test_boundary_max_u64_wrong_serialize_length() {
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(spec_u64_to_le_bytes(u64::MAX).len() != 8);
}

} // end verus!
