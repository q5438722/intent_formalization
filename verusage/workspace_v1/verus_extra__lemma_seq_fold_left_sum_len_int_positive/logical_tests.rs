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

// === LOGICAL TESTS ===
// These test properties NOT explicitly guaranteed by the spec.
// Each should FAIL because the spec does not entail these properties.

// Test 1: Stronger lower bound — fold result >= low
// The ensures only says >= 0, not >= low. While this IS mathematically true
// (since low >= 0 and f(x).len() >= 0), the spec's ensures clause alone
// only provides >= 0, which is insufficient to derive >= low for arbitrary low.
// SHOULD FAIL
proof fn test_logical_result_at_least_low(s: Seq<int>, low: nat, f: spec_fn(int) -> Seq<int>)
    ensures
        s.fold_left(low as int, |acc: int, x: int| acc + f(x).len()) >= low as int,
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low, f);
}

// Test 2: Monotonicity in low — increasing low increases the fold result
// The spec makes no relational guarantee between two different calls.
// While mathematically true, the ensures clause (>= 0) says nothing about
// how changing low affects the result.
// SHOULD FAIL
proof fn test_logical_monotone_in_low(s: Seq<int>, low1: nat, low2: nat, f: spec_fn(int) -> Seq<int>)
    requires
        low1 < low2,
    ensures
        s.fold_left(low1 as int, |acc: int, x: int| acc + f(x).len())
          < s.fold_left(low2 as int, |acc: int, x: int| acc + f(x).len()),
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low1, f);
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low2, f);
}

// Test 3: Upper bound — the fold result is bounded above by low + s.len() * N
// The spec provides no upper bound at all. We cannot derive any bound from >= 0.
// SHOULD FAIL
proof fn test_logical_upper_bound(s: Seq<int>, low: nat, f: spec_fn(int) -> Seq<int>)
    requires
        s.len() > 0,
    ensures
        s.fold_left(low as int, |acc: int, x: int| acc + f(x).len()) <= low as int + s.len() * 100,
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low, f);
}

}
