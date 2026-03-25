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

// BEHAVIORAL MUTATION TEST 1: SHOULD FAIL
// Off-by-one positive mutation: assert fold_seq.len() == fold_int + 1
// The lemma says fold_seq.len() == fold_int exactly, so +1 is rejected
proof fn test_mutation_1_plus_one(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        s.fold_left(prefix.len() as int, |i: int, a: int| i + f(a).len() as int) + 1
    ); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 2: SHOULD FAIL
// Off-by-one negative mutation: assert fold_seq.len() == fold_int - 1
// The lemma says fold_seq.len() == fold_int exactly, so -1 is rejected
proof fn test_mutation_2_minus_one(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        s.fold_left(prefix.len() as int, |i: int, a: int| i + f(a).len() as int) - 1
    ); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 3: SHOULD FAIL
// Wrong initial value mutation: use 0 instead of prefix.len() as the fold base
// With prefix.len() > 0, the fold from 0 differs from fold from prefix.len()
proof fn test_mutation_3_wrong_initial_value(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
    requires prefix.len() > 0,
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        s.fold_left(0int, |i: int, a: int| i + f(a).len() as int)
    ); // SHOULD FAIL
}

}
