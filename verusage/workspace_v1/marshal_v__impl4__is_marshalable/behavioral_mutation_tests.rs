use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ============================================================
// Definitions from target file
// ============================================================

#[verifier::external_body]
pub proof fn lemma_seq_fold_left_sum_right<A>(s: Seq<A>, low: int, f: spec_fn(A) -> int)
  requires
    s.len() > 0,
  ensures
    s.subrange(0, s.len() - 1).fold_left(low, |b: int, a: A| b + f(a)) + f(s[s.len() - 1])
    ==
    s.fold_left(low, |b: int, a: A| b + f(a))
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn lemma_seq_fold_left_append_len_int_le<A, B>(s: Seq<A>, i: int, low: int, f: spec_fn(A) -> Seq<B>)
  requires
    0 <= i <= s.len() as int,
    0 <= low,
  ensures
    s.fold_left(low, |acc: int, x: A| acc + f(x).len()) >= 0,
    s.subrange(0, i).fold_left(low, |acc: int, x: A| acc + f(x).len()) <=
    s.fold_left(low, |acc: int, x: A| acc + f(x).len()),
  decreases (2 * s.len() - i),
{
    unimplemented!()
}

pub trait Marshalable : Sized {
  spec fn is_marshalable(&self) -> bool;

  #[verifier::external_body]
  exec fn _is_marshalable(&self) -> (res: bool)
    ensures res == self.is_marshalable()
  { unimplemented!() }

  #[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  { unimplemented!() }

  #[verifier::external_body]
  exec fn serialized_size(&self) -> (res: usize)
    requires self.is_marshalable(),
    ensures res as int == self.ghost_serialize().len(),
  { unimplemented!() }
}

impl Marshalable for u64 {
  open spec fn is_marshalable(&self) -> bool { true }

  #[verifier::external_body]
  exec fn _is_marshalable(&self) -> (res: bool) { unimplemented!() }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

  #[verifier::external_body]
  #[verifier::spinoff_prover]
  exec fn serialized_size(&self) -> (res: usize) { unimplemented!() }
}

impl Marshalable for usize {
  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  #[verifier::external_body]
  exec fn _is_marshalable(&self) -> (res: bool) { unimplemented!() }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

  #[verifier::external_body]
  exec fn serialized_size(&self) -> (res: usize) { unimplemented!() }
}

impl Marshalable for Vec<u8> {
  open spec fn is_marshalable(&self) -> bool {
    self@.len() <= usize::MAX &&
    (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
  }

  #[verifier::external_body]
  exec fn _is_marshalable(&self) -> (res: bool) { unimplemented!() }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (self@.len() as usize).ghost_serialize()
      + self@
  }

  #[verifier::external_body]
  exec fn serialized_size(&self) -> (res: usize) { unimplemented!() }
}


// ============================================================
// BEHAVIORAL MUTATION TESTS: Mutated expected outputs/relations
// These should all FAIL - testing that incorrect behaviors are rejected
// ============================================================

// Test M1: Assert fold_left sum decomposition gives wrong result (off by one)
// The lemma ensures exact equality; adding +1 should be rejected
proof fn test_m1_fold_sum_off_by_one() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    // Lemma gives: subrange(0,2).fold_left(0, +f) + f(s[2]) == s.fold_left(0, +f)
    // Mutate: claim the result is off by one
    assert(
        s.subrange(0, s.len() - 1).fold_left(0int, |b: int, a: int| b + f(a)) + f(s[s.len() - 1]) + 1
        ==
        s.fold_left(0int, |b: int, a: int| b + f(a))
    );
}

// Test M2: Assert the fold_left inequality is reversed (subrange > whole)
// The lemma ensures subrange fold <= whole fold; reversing should fail
proof fn test_m2_reversed_inequality() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![0u8; x as nat]; // sequence of length x
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 1, 0, f);
    // Lemma ensures subrange(0,1).fold_left(...) <= s.fold_left(...)
    // Mutate: assert strict greater-than
    assert(
        s.subrange(0, 1).fold_left(0int, |acc: int, x: int| acc + f(x).len())
        >
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

// Test M3: Assert u64 ghost_serialize produces empty sequence
// u64 serialization should produce 8 bytes, not 0
proof fn test_m3_u64_serialize_empty() { // SHOULD FAIL
    let v: u64 = 42u64;
    assert(v.ghost_serialize().len() == 0);
}

// Test M4: Assert usize serialization differs from its u64 cast
// usize.ghost_serialize() is defined as (*self as u64).ghost_serialize()
proof fn test_m4_usize_u64_mismatch() { // SHOULD FAIL
    let v: usize = 100usize;
    let u: u64 = 100u64;
    // They should be equal by definition; assert inequality
    assert(v.ghost_serialize() !== u.ghost_serialize());
}

// Test M5: Assert Vec<u8> serialization does NOT start with length prefix
// Vec<u8>.ghost_serialize() = len_prefix + data; claim first byte is data
proof fn test_m5_vec_u8_no_length_prefix() { // SHOULD FAIL
    let data: Seq<u8> = seq![0xAAu8, 0xBBu8];
    // ghost_serialize = (len as usize).ghost_serialize() + data
    // The length prefix for len=2 is (2usize as u64).ghost_serialize() = 8 bytes
    // So ghost_serialize().len() should be 10, not 2
    // Claim serialized length equals just the data length (no prefix)
    assert(
        (2usize).ghost_serialize().len() + 2 == 2
    );
}

}
