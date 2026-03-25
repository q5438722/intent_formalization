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

// === BOUNDARY TESTS ===
// These tests violate the precondition (requires clause).
// They should all FAIL verification because the requires is not satisfied.

// Test 1: f(s[i]) > high for all elements
// SHOULD FAIL
proof fn test_boundary_f_exceeds_high() {
    let s: Seq<int> = seq![10int, 20, 30];
    // f = identity, so f(10)=10, f(20)=20, f(30)=30
    // high = 5, but all f(s[i]) > 5 => requires violated
    lemma_seq_fold_left_sum_le::<int>(s, 0, 5, |x: int| x);
}

// Test 2: high = 0 but f returns positive values
// SHOULD FAIL
proof fn test_boundary_high_zero_positive_f() {
    let s: Seq<int> = seq![1int, 2, 3];
    // f = identity, so f(1)=1, f(2)=2, f(3)=3
    // high = 0, but all f(s[i]) > 0 => requires violated
    lemma_seq_fold_left_sum_le::<int>(s, 0, 0, |x: int| x);
}

// Test 3: Negative high, but f values exceed it
// SHOULD FAIL
proof fn test_boundary_negative_high_exceeded() {
    let s: Seq<int> = seq![1int, 2];
    // f = identity, so f(1)=1, f(2)=2
    // high = -10, but f(s[i]) > -10 (1 > -10, but 1 <= -10 is false) => requires violated
    lemma_seq_fold_left_sum_le::<int>(s, 0, -10, |x: int| x);
}

}
