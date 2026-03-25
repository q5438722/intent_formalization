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
// LOGICAL TESTS: Properties NOT explicitly guaranteed
// These should all FAIL - testing if spec allows unintended reasoning
// ============================================================

// Test L1: Assert u64 serialization is injective (different values => different serializations)
// The spec does not guarantee injectivity; it only defines serialization via spec_u64_to_le_bytes
proof fn test_l1_u64_serialize_injective() { // SHOULD FAIL
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // Attempt to prove injectivity - spec doesn't explicitly guarantee this
    assert(a.ghost_serialize() !== b.ghost_serialize());
}

// Test L2: Assert fold_left is strictly monotone (subrange < whole, not just <=)
// The lemma only guarantees <=, so strict < should not always hold
proof fn test_l2_strict_monotonicity() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| Seq::<u8>::empty(); // f produces empty sequences, so fold adds 0
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 0, 0, f);
    // With f producing empty seqs, subrange(0,0).fold_left == s.fold_left (both are 0)
    // But we assert strict inequality
    assert(
        s.subrange(0, 0).fold_left(0int, |acc: int, x: int| acc + f(x).len())
        <
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

// Test L3: Assert fold_left sum is commutative over sequence reordering
// The spec says nothing about commutativity of fold_left over different orderings
proof fn test_l3_fold_commutativity() { // SHOULD FAIL
    let s1: Seq<int> = seq![1int, 2int, 3int];
    let s2: Seq<int> = seq![3int, 1int, 2int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s1, 0, f);
    lemma_seq_fold_left_sum_right::<int>(s2, 0, f);
    // Claim fold results are equal for reordered sequences
    // The spec doesn't prove this (even though addition is commutative)
    assert(
        s1.fold_left(0int, |b: int, a: int| b + f(a))
        ==
        s2.fold_left(0int, |b: int, a: int| b + f(a))
    );
}

// Test L4: Assert that fold_left with low=0 gives same result as fold_left with low=1
// The spec doesn't claim independence from the initial accumulator
proof fn test_l4_fold_low_independence() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    lemma_seq_fold_left_sum_right::<int>(s, 1, f);
    // Assert fold with low=0 equals fold with low=1 (should differ by 1)
    assert(
        s.fold_left(0int, |b: int, a: int| b + f(a))
        ==
        s.fold_left(1int, |b: int, a: int| b + f(a))
    );
}

// Test L5: Assert ghost_serialize length is always exactly 8 for usize
// The spec defines usize serialization as u64 serialization, but doesn't
// explicitly guarantee the length; spec_u64_to_le_bytes axioms may or may not provide this
proof fn test_l5_usize_serialize_length_always_8() { // SHOULD FAIL
    let v: usize = 42usize;
    // This property requires knowledge about spec_u64_to_le_bytes producing 8 bytes
    // The spec doesn't explicitly state this
    assert(v.ghost_serialize().len() == 8);
}

}
