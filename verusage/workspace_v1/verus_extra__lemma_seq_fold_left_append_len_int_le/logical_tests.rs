use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// === Original lemma declarations (external_body for testing spec interface) ===

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
pub proof fn lemma_seq_fold_left_sum_len_int_positive<A, B>(s: Seq<A>, low: nat, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(low as int, |acc: int, x: A| acc + f(x).len()) >= 0,
  decreases s.len(),
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
{
    unimplemented!()
}

// === LOGICAL TESTS ===
// These test properties NOT explicitly guaranteed by the specification.

// Test 1: Assert strict inequality when i == s.len() (spec only guarantees <=, and here they are equal)
// SHOULD FAIL
proof fn test_logical_strict_inequality_at_len() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 3int, 0int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    // When i == s.len(), subrange(0, len) == s, so folds are equal. Strict < should fail.
    assert(s.subrange(0, 3).fold_left(0int, accfl) < s.fold_left(0int, accfl));
}

// Test 2: Assert fold is independent of low (changing low should change the fold result)
// SHOULD FAIL
proof fn test_logical_low_independence() {
    let s: Seq<int> = seq![1int, 2int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1int, 0int, f);
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1int, 10int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    // fold(0, ...) = 0 + 1 + 1 = 2, fold(10, ...) = 10 + 1 + 1 = 12
    // They are NOT equal, so asserting equality should fail.
    assert(s.fold_left(0int, accfl) == s.fold_left(10int, accfl));
}

// Test 3: Assert fold result equals low for non-empty sequence (not guaranteed)
// SHOULD FAIL
proof fn test_logical_fold_equals_low() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 3int, 5int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    // fold(5, ...) = 5 + 1 + 1 + 1 = 8, not 5. Assert fold == low should fail.
    assert(s.fold_left(5int, accfl) == 5int);
}

// Test 4: Assert the same inequality holds for a subtraction-based accumulator (not guaranteed)
// The spec only guarantees the property for |acc, x| acc + f(x).len(), not for other accumulators.
// SHOULD FAIL
proof fn test_logical_different_accumulator() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    let sub_accfl = |acc: int, x: int| -> int { acc - f(x).len() };
    // The spec says nothing about subtraction-based accumulator.
    // subrange(0,2).fold_left(0, sub) = 0 - 1 - 1 = -2
    // full.fold_left(0, sub) = 0 - 1 - 1 - 1 = -3
    // -2 <= -3 is FALSE, so asserting it should fail.
    assert(s.subrange(0, 2).fold_left(0int, sub_accfl) <= s.fold_left(0int, sub_accfl));
}

}
