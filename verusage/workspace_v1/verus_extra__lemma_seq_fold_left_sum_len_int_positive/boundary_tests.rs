use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemma under test
pub proof fn lemma_seq_fold_left_sum_len_int_positive<A, B>(s: Seq<A>, low: nat, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(low as int, |acc: int, x: A| acc + f(x).len()) >= 0,
  decreases s.len(),
{
  s.lemma_fold_left_alt(low as int, |acc: int, x: A| acc + f(x).len());
  if s.len() != 0 {
    lemma_seq_fold_left_sum_len_int_positive::<A, B>(s.subrange(1, s.len() as int), low + f(s[0]).len(), f);
    s.subrange(1, s.len() as int).lemma_fold_left_alt(low + f(s[0]).len() as int, |acc: int, x: A| acc + f(x).len());
  }
}

// === BOUNDARY TESTS ===
// The lemma has NO requires clause. These tests probe edge cases
// to check if the postcondition is overly permissive at boundaries.

// Test 1: Empty sequence with low=0 — fold result is exactly 0, not > 0
// fold_left on empty seq returns the initial value (0). Strict positivity is false.
// SHOULD FAIL
proof fn test_boundary_empty_seq_strict_positive() {
    let s: Seq<int> = Seq::empty();
    let f = |x: int| Seq::<int>::empty();
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, 0, f);
    assert(s.fold_left(0int, |acc: int, x: int| acc + f(x).len()) > 0);
}

// Test 2: Non-empty seq where f maps every element to an empty seq, low=0
// All f(x).len() = 0, so fold = 0. Asserting > 0 should fail.
// SHOULD FAIL
proof fn test_boundary_nonempty_all_zero_lengths() {
    let s: Seq<int> = seq![1int, 2, 3];
    let f = |x: int| Seq::<int>::empty();
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, 0, f);
    assert(s.fold_left(0int, |acc: int, x: int| acc + f(x).len()) > 0);
}

// Test 3: Empty seq with low=1 — fold result is 1, asserting < 1 is false
// SHOULD FAIL
proof fn test_boundary_empty_seq_below_low() {
    let s: Seq<int> = Seq::empty();
    let f = |x: int| Seq::<int>::empty();
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, 1, f);
    assert(s.fold_left(1int, |acc: int, x: int| acc + f(x).len()) < 1);
}

}
