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

// === LOGICAL TESTS ===
// These test properties NOT explicitly guaranteed by the spec.
// They should all FAIL because the spec does not entail them.

// Test 1: Lower bound assumption — fold >= init
// Not guaranteed: f can return negative values (f(s[i]) <= high allows f(s[i]) < 0)
// Counterexample: f(x) = -100, high = 0, then fold = init + sum_of_negatives < init
// SHOULD FAIL
proof fn test_logical_lower_bound(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
        s.len() > 0,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) >= init,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

// Test 2: Non-negative result — fold >= 0
// Not guaranteed: with negative init and/or negative f values, fold can be negative
// Counterexample: init = -100, f(x) = -50, high = 0
// SHOULD FAIL
proof fn test_logical_non_negative(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
        s.len() > 0,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) >= 0,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

// Test 3: Exact equality — fold == init + len * high
// Spec only guarantees <=, not ==. Equality requires all f(s[i]) == high exactly.
// Counterexample: f(s[i]) < high for any i
// SHOULD FAIL
proof fn test_logical_exact_equality(s: Seq<int>, init: int, high: int, f: spec_fn(int) -> int)
    requires
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= high,
        s.len() > 0,
    ensures
        s.fold_left(init, |acc: int, x: int| acc + f(x)) == init + s.len() * high,
{
    lemma_seq_fold_left_sum_le::<int>(s, init, high, f);
}

}
