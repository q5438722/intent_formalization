use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

pub proof fn lemma_seq_fold_left_append_len_int<A, B>(s: Seq<A>, prefix: Seq<B>, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(prefix, |sb: Seq<B>, a: A| sb + f(a)).len() as int
    ==
    s.fold_left(prefix.len() as int, |i: int, a: A| i + f(a).len() as int),
  decreases s.len(),
{
  s.lemma_fold_left_alt(prefix, |sb: Seq<B>, a: A| sb + f(a));
  s.lemma_fold_left_alt(prefix.len() as int, |i: int, a: A| i + f(a).len() as int);
  if s.len() != 0 {
    lemma_seq_fold_left_append_len_int::<A, B>(s.subrange(1, s.len() as int), prefix + f(s[0]), f);
    s.subrange(1, s.len() as int).lemma_fold_left_alt(prefix + f(s[0]), |sb: Seq<B>, a: A| sb + f(a));
    s.subrange(1, s.len() as int).lemma_fold_left_alt(prefix.len() as int + f(s[0]).len() as int, |i: int, a: A| i + f(a).len() as int);
  }
}

// LOGICAL TEST 1: SHOULD FAIL
// Assert fold length always equals prefix.len() regardless of s
// This claims f contributes nothing, which is not entailed by the spec
proof fn test_logical_1_independent_of_s(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        prefix.len() as int
    ); // SHOULD FAIL
}

// LOGICAL TEST 2: SHOULD FAIL
// Assert fold with different f functions gives same length (f-independence)
// The spec does not guarantee that different f's produce same total length
proof fn test_logical_2_f_independence(s: Seq<int>, prefix: Seq<int>, f1: spec_fn(int) -> Seq<int>, f2: spec_fn(int) -> Seq<int>)
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f1);
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f2);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f1(a)).len() as int
        ==
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f2(a)).len() as int
    ); // SHOULD FAIL
}

// LOGICAL TEST 3: SHOULD FAIL
// Assert fold length is strictly greater than prefix.len() for all inputs
// This stronger inequality is not guaranteed (fails for empty s or f returning empty seqs)
proof fn test_logical_3_strict_inequality(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        >
        prefix.len() as int
    ); // SHOULD FAIL
}

}
