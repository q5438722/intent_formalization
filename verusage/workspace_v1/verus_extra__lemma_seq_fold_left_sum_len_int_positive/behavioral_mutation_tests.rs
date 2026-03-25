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

// === BEHAVIORAL MUTATION TESTS ===
// These tests mutate the postcondition to a stronger or different relation.
// Each should FAIL because the mutated property is not entailed by the spec.

// Test 1: Mutate >= 0 to > 0 (strict positivity)
// Counterexample: empty seq with low=0 gives fold=0, which is NOT > 0
// SHOULD FAIL
proof fn test_mutation_strict_positive(s: Seq<int>, low: nat, f: spec_fn(int) -> Seq<int>)
    ensures
        s.fold_left(low as int, |acc: int, x: int| acc + f(x).len()) > 0,
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low, f);
}

// Test 2: Mutate >= 0 to >= low + 1 (result must exceed low)
// Counterexample: empty seq gives fold = low, which is NOT >= low + 1
// SHOULD FAIL
proof fn test_mutation_above_low_plus_one(s: Seq<int>, low: nat, f: spec_fn(int) -> Seq<int>)
    ensures
        s.fold_left(low as int, |acc: int, x: int| acc + f(x).len()) >= low as int + 1,
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low, f);
}

// Test 3: Mutate >= 0 to == 0 (fold is exactly zero)
// Counterexample: low > 0 gives fold >= low > 0
// SHOULD FAIL
proof fn test_mutation_exact_zero(s: Seq<int>, low: nat, f: spec_fn(int) -> Seq<int>)
    ensures
        s.fold_left(low as int, |acc: int, x: int| acc + f(x).len()) == 0,
{
    lemma_seq_fold_left_sum_len_int_positive::<int, int>(s, low, f);
}

}
