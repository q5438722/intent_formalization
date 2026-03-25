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

// Boundary Test 1: Empty sequence violates requires s.len() > 0
// SHOULD FAIL
proof fn test_empty_seq_sum_right() {
    let s: Seq<int> = Seq::<int>::empty();
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
}

// Boundary Test 2: Negative index violates requires 0 <= i
// SHOULD FAIL
proof fn test_negative_index() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![x];
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, -1, 0, f);
}

// Boundary Test 3: Index exceeds length violates requires i <= s.len()
// SHOULD FAIL
proof fn test_index_exceeds_length() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![x];
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 4, 0, f);
}

// Boundary Test 4: Negative low violates requires 0 <= low
// SHOULD FAIL
proof fn test_negative_low() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![x];
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1, -1, f);
}

}
