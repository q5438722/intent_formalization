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


// ============================================================
// BOUNDARY TESTS: Precondition Violations
// These should all FAIL - testing that preconditions are enforced
// ============================================================

// Test B1: Call lemma_seq_fold_left_sum_right with empty sequence
// Violates requires s.len() > 0
proof fn test_b1_empty_seq_sum_right() { // SHOULD FAIL
    let s: Seq<int> = Seq::empty();
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
}

// Test B2: Call lemma_seq_fold_left_append_len_int_le with i < 0
// Violates requires 0 <= i
proof fn test_b2_negative_index() { // SHOULD FAIL
    let s: Seq<int> = seq![1int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, -1int, 0, f);
}

// Test B3: Call lemma_seq_fold_left_append_len_int_le with i > s.len()
// Violates requires i <= s.len()
proof fn test_b3_index_exceeds_len() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 3int, 0, f);
}

// Test B4: Call lemma_seq_fold_left_append_len_int_le with negative low
// Violates requires 0 <= low
proof fn test_b4_negative_low() { // SHOULD FAIL
    let s: Seq<int> = seq![1int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 0, -1int, f);
}

// Test B5: Call lemma_seq_fold_left_append_len_int_le on empty seq with i=1
// Violates requires i <= s.len() (0)
proof fn test_b5_empty_seq_nonzero_index() { // SHOULD FAIL
    let s: Seq<int> = Seq::empty();
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 1int, 0, f);
}

}
