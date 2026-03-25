use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemma under test
pub proof fn lemma_seq_fold_left_sum_le<A>(s: Seq<A>, init: int, high: int, f: spec_fn(A) -> int)
  requires
    forall |i:int| 0 <= i < s.len() ==> f(s[i]) <= high,
  ensures
    s.fold_left(init, |acc: int, x: A| acc + f(x)) <= init + s.len() * high,
  decreases s.len(),
{
  if s.len() != 0 {
    lemma_seq_fold_left_sum_le(s.drop_last(), init, high, f);
    assert(init + (s.len() - 1) * high + high <= init + s.len() * high) by (nonlinear_arith);
  }
}

// === BEHAVIORAL MUTATION TESTS ===
// These tests mutate the postcondition to a stronger or different relation.
// They should all FAIL because the mutated property is not entailed by the spec.

// Test 1: Mutate <= to < (strict inequality)
// Counterexample: when f(s[i]) == high for all i, fold == init + len*high (equality holds)
// SHOULD FAIL
proof fn test_mutation_strict_inequality(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) < init + s.len() * high,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

// Test 2: Tighter bound using (s.len() - 1) instead of s.len()
// Counterexample: single-element seq with f(s[0]) = high gives fold = init + high > init + 0*high
// SHOULD FAIL
proof fn test_mutation_tighter_bound(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
        s.len() > 0,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) <= init + (s.len() - 1) * high,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

// Test 3: Reversed inequality (>= instead of <=)
// Counterexample: f(s[i]) = 0 < high gives fold = init < init + len*high
// SHOULD FAIL
proof fn test_mutation_reversed_inequality(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
        s.len() > 0,
        high > 0,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) >= init + s.len() * high,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

}
