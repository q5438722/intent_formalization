use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ===== Original lemmas (from target file) =====

#[verifier::external_body]
pub proof fn lemma_seq_add_subrange<A>(s: Seq<A>, i: int, j: int, k: int)
  requires 0 <= i <= j <= k <= s.len(),
  ensures s.subrange(i, j) + s.subrange(j, k) == s.subrange(i, k),
{
    unimplemented!()
}

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
pub proof fn lemma_seq_fold_left_append_right<A, B>(s: Seq<A>, prefix: Seq<B>, f: spec_fn(A) -> Seq<B>)
  requires s.len() > 0,
  ensures
    s.subrange(0, s.len() - 1).fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a)) + f(s[s.len() - 1])
    ==
    s.fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a))
{
    unimplemented!()
}

// ========== BOUNDARY TESTS ==========

// Test 1: Violate i <= j ordering in lemma_seq_add_subrange
// i=2 > j=1 violates requires 0 <= i <= j <= k <= s.len()
// SHOULD FAIL
proof fn test_boundary_subrange_i_greater_than_j() {
    let s = seq![1u8, 2u8, 3u8];
    lemma_seq_add_subrange::<u8>(s, 2, 1, 3);
}

// Test 2: Violate k <= s.len() in lemma_seq_add_subrange
// k=4 exceeds s.len()=3
// SHOULD FAIL
proof fn test_boundary_subrange_k_exceeds_len() {
    let s = seq![1u8, 2u8, 3u8];
    lemma_seq_add_subrange::<u8>(s, 0, 1, 4);
}

// Test 3: Violate 0 <= i with negative index
// i=-1 violates the lower bound
// SHOULD FAIL
proof fn test_boundary_subrange_negative_index() {
    let s = seq![1u8, 2u8, 3u8];
    lemma_seq_add_subrange::<u8>(s, -1, 1, 3);
}

// Test 4: Violate s.len() > 0 in lemma_seq_fold_left_sum_right
// Empty sequence violates the precondition
// SHOULD FAIL
proof fn test_boundary_fold_sum_empty_seq() {
    let s = Seq::<u64>::empty();
    let f = |x: u64| x as int;
    lemma_seq_fold_left_sum_right::<u64>(s, 0, f);
}

// Test 5: Violate s.len() > 0 in lemma_seq_fold_left_append_right
// Empty sequence violates the precondition
// SHOULD FAIL
proof fn test_boundary_fold_append_empty_seq() {
    let s = Seq::<u64>::empty();
    let prefix = Seq::<u8>::empty();
    let f = |x: u64| spec_u64_to_le_bytes(x);
    lemma_seq_fold_left_append_right::<u64, u8>(s, prefix, f);
}

}
