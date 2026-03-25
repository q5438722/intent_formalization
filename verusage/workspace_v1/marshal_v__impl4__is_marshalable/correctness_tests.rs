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
// BOUNDARY TESTS (B1-B5): Precondition violations
// ============================================================

// B1: Empty sequence violates requires s.len() > 0
proof fn test_b1_empty_seq_sum_right() { // SHOULD FAIL
    let s: Seq<int> = Seq::empty();
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
}

// B2: Negative index violates requires 0 <= i
proof fn test_b2_negative_index() { // SHOULD FAIL
    let s: Seq<int> = seq![1int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, -1int, 0, f);
}

// B3: Index exceeding length violates requires i <= s.len()
proof fn test_b3_index_exceeds_len() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 3int, 0, f);
}

// B4: Negative accumulator violates requires 0 <= low
proof fn test_b4_negative_low() { // SHOULD FAIL
    let s: Seq<int> = seq![1int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 0, -1int, f);
}

// B5: Non-zero index on empty sequence violates i <= s.len()
proof fn test_b5_empty_seq_nonzero_index() { // SHOULD FAIL
    let s: Seq<int> = Seq::empty();
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 1int, 0, f);
}


// ============================================================
// BEHAVIORAL MUTATION TESTS (M1-M5): Mutated outputs/relations
// ============================================================

// M1: Fold sum off by one (mutated equality)
proof fn test_m1_fold_sum_off_by_one() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    assert(
        s.subrange(0, s.len() - 1).fold_left(0int, |b: int, a: int| b + f(a)) + f(s[s.len() - 1]) + 1
        ==
        s.fold_left(0int, |b: int, a: int| b + f(a))
    );
}

// M2: Reversed inequality (subrange > whole)
proof fn test_m2_reversed_inequality() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![0u8; x as nat];
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 1, 0, f);
    assert(
        s.subrange(0, 1).fold_left(0int, |acc: int, x: int| acc + f(x).len())
        >
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

// M3: u64 serialize produces 0 bytes (wrong length)
proof fn test_m3_u64_serialize_empty() { // SHOULD FAIL
    let v: u64 = 42u64;
    assert(v.ghost_serialize().len() == 0);
}

// M4: usize/u64 serialize mismatch (contradicts definition)
proof fn test_m4_usize_u64_mismatch() { // SHOULD FAIL
    let v: usize = 100usize;
    let u: u64 = 100u64;
    assert(v.ghost_serialize() !== u.ghost_serialize());
}

// M5: Vec<u8> serialize has no length prefix (wrong total size)
proof fn test_m5_vec_u8_no_length_prefix() { // SHOULD FAIL
    assert(
        (2usize).ghost_serialize().len() + 2 == 2
    );
}


// ============================================================
// LOGICAL TESTS (L1-L5): Properties NOT guaranteed by spec
// ============================================================

// L1: u64 serialize injectivity (not explicitly guaranteed)
proof fn test_l1_u64_serialize_injective() { // SHOULD FAIL
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    assert(a.ghost_serialize() !== b.ghost_serialize());
}

// L2: Strict monotonicity of fold (spec only guarantees <=)
proof fn test_l2_strict_monotonicity() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| Seq::<u8>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, u8>(s, 0, 0, f);
    assert(
        s.subrange(0, 0).fold_left(0int, |acc: int, x: int| acc + f(x).len())
        <
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

// L3: Fold commutativity over reordering (not provable from spec)
proof fn test_l3_fold_commutativity() { // SHOULD FAIL
    let s1: Seq<int> = seq![1int, 2int, 3int];
    let s2: Seq<int> = seq![3int, 1int, 2int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s1, 0, f);
    lemma_seq_fold_left_sum_right::<int>(s2, 0, f);
    assert(
        s1.fold_left(0int, |b: int, a: int| b + f(a))
        ==
        s2.fold_left(0int, |b: int, a: int| b + f(a))
    );
}

// L4: Fold independence from initial accumulator (clearly false)
proof fn test_l4_fold_low_independence() { // SHOULD FAIL
    let s: Seq<int> = seq![1int, 2int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    lemma_seq_fold_left_sum_right::<int>(s, 1, f);
    assert(
        s.fold_left(0int, |b: int, a: int| b + f(a))
        ==
        s.fold_left(1int, |b: int, a: int| b + f(a))
    );
}

// L5: usize serialize always 8 bytes (requires axiom about spec_u64_to_le_bytes)
proof fn test_l5_usize_serialize_length_always_8() { // SHOULD FAIL
    let v: usize = 42usize;
    assert(v.ghost_serialize().len() == 8);
}

}
