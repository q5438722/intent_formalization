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

// Behavioral Mutation Test 1: u64 serialization length is 4 (wrong, should be 8)
// SHOULD FAIL
proof fn test_u64_serialize_wrong_length() {
    let v: u64 = 42;
    assert(spec_u64_to_le_bytes(v).len() == 4);
}

// Behavioral Mutation Test 2: Mutate sum_right relation from addition to subtraction
// SHOULD FAIL
proof fn test_sum_right_subtraction() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| x;
    lemma_seq_fold_left_sum_right::<int>(s, 0, f);
    // Postcondition gives: subrange.fold + f(last) == full.fold
    // Mutated: subtract instead of add
    assert(
        s.subrange(0, s.len() - 1).fold_left(0int, |b: int, a: int| b + f(a)) - f(s[s.len() - 1])
        ==
        s.fold_left(0int, |b: int, a: int| b + f(a))
    );
}

// Behavioral Mutation Test 3: fold_append_len_int equality off-by-one
// SHOULD FAIL
proof fn test_fold_append_len_off_by_one() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let prefix = Seq::<int>::empty();
    let f = |x: int| seq![x];
    lemma_seq_fold_left_append_len_int::<int, int>(s, prefix, f);
    // Postcondition gives: fold_append.len == fold_sum
    // Mutated: off-by-one on the RHS
    assert(
        s.fold_left(prefix, |sb: Seq<int>, a: int| sb + f(a)).len() as int
        ==
        s.fold_left(prefix.len() as int, |i: int, a: int| i + f(a).len() as int) + 1
    );
}

// Behavioral Mutation Test 4: Reversed inequality (> instead of <=)
// SHOULD FAIL
proof fn test_fold_le_reversed() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| seq![x];
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1, 0, f);
    // Postcondition gives: subrange(0,i).fold <= full.fold
    // Mutated: assert strict > (reversed)
    assert(
        s.subrange(0, 1).fold_left(0int, |acc: int, x: int| acc + f(x).len()) >
        s.fold_left(0int, |acc: int, x: int| acc + f(x).len())
    );
}

}
