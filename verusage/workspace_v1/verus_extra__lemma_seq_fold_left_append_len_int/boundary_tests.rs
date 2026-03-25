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

// BOUNDARY TEST 1: SHOULD FAIL
// Empty sequence edge case: assert fold length is prefix.len() + 1 (off-by-one at boundary)
// For empty s, fold returns prefix, so fold.len() = prefix.len(), NOT prefix.len() + 1
proof fn test_boundary_1_empty_seq_off_by_one(prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
{
    let s = Seq::<int>::empty();
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        prefix.len() as int + 1
    ); // SHOULD FAIL
}

// BOUNDARY TEST 2: SHOULD FAIL
// Non-empty prefix with empty s: assert fold length is 0 (ignoring prefix contribution)
// For empty s, fold returns prefix, so fold.len() = prefix.len() > 0, NOT 0
proof fn test_boundary_2_nonempty_prefix_wrong_zero(prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
    requires prefix.len() > 0,
{
    let s = Seq::<int>::empty();
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int == 0int
    ); // SHOULD FAIL
}

// BOUNDARY TEST 3: SHOULD FAIL
// Non-empty s: assert fold length is strictly less than prefix.len()
// Since fold appends sequences via f, result should be >= prefix.len(), NOT less
proof fn test_boundary_3_fold_shorter_than_prefix(s: Seq<int>, prefix: Seq<int>, f: spec_fn(int) -> Seq<int>)
    requires s.len() > 0,
{
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    let fold_len = s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int;
    let prefix_len = prefix.len() as int;
    assert(fold_len < prefix_len); // SHOULD FAIL
}

}
