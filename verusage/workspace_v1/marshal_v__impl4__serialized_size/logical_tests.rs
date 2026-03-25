use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

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
pub proof fn lemma_seq_fold_left_append_len_int<A, B>(s: Seq<A>, prefix: Seq<B>, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a)).len() as int
    ==
    s.fold_left(prefix.len() as int, |i: int, a: A| i + f(a).len() as int),
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
  decreases (2 * s.len() - i),
{
    unimplemented!()
}

// Logical Test 1: Check that sum_right lemma is not unsound (assert false after call)
// SHOULD FAIL
proof fn test_sum_right_not_unsound() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    assert(false);
}

// Logical Test 2: Assert fold result is strictly > low (stronger than >= 0 from spec)
// SHOULD FAIL
proof fn test_fold_strictly_greater_than_low() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| Seq::<int>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 3, 5, f);
    // Lemma ensures fold(5, ...) >= 0, not fold(5, ...) > 5
    // With f returning empty seqs, fold should stay at 5
    assert(s.fold_left(5int, |acc: int, x: int| acc + f(x).len()) > 5);
}

// Logical Test 3: Assert strict < for subrange instead of <= from spec
// SHOULD FAIL
proof fn test_subrange_strict_less_than() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| Seq::<int>::empty();
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1, 0, f);
    // Lemma ensures subrange(0,1).fold <= full.fold
    // With f returning empty, both sides equal 0, so strict < should fail
    assert(
        s.subrange(0, 1).fold_left(0int, |acc: int, x: int| acc + f(x).len()) <
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

// Logical Test 4: Cross-function misuse - equate results from different lemmas
// SHOULD FAIL
proof fn test_cross_function_misuse() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f_int = |x: int| x;
    let f_seq = |x: int| seq![x];
    lemma_seq_fold_left_sum_right::<int>(s, 0, f_int);
    lemma_seq_fold_left_append_len_int::<int, int>(s, Seq::<int>::empty(), f_seq);
    // sum_right tells us about fold(0, |b,a| b+a, s)
    // append_len tells us fold(empty, append, s).len == fold(0, |i,a| i+1, s)
    // Assert they are equal (sum of values == count of elements) - wrong in general
    assert(
        s.fold_left(0int, |b: int, a: int| b + f_int(a))
        ==
        s.fold_left(Seq::<int>::empty(), |sb: Seq<int>, a: int| sb + f_seq(a)).len()
    );
}

}
