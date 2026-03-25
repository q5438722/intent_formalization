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

// === BEHAVIORAL MUTATION TESTS ===
// These tests start from valid inputs but assert incorrect output relations.

// Test 1: Assert fold result is negative (contradicts ensures: fold >= 0)
// SHOULD FAIL
proof fn test_mutation_fold_negative() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 2int, 0int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    assert(s.fold_left(0int, accfl) < 0int);
}

// Test 2: Assert prefix fold > full fold (contradicts ensures: prefix <= full)
// SHOULD FAIL
proof fn test_mutation_prefix_greater_than_full() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 2int, 0int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    assert(s.subrange(0, 2).fold_left(0int, accfl) > s.fold_left(0int, accfl));
}

// Test 3: Assert sum_right decomposition is NOT equal (contradicts ensures of sum_right)
// SHOULD FAIL
proof fn test_mutation_sum_right_not_equal() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> int { x };
    lemma_seq_fold_left_sum_right::<int>(s, 0int, f);
    let accf = |b: int, a: int| -> int { b + f(a) };
    assert(
        s.subrange(0, s.len() - 1).fold_left(0int, accf) + f(s[s.len() - 1])
        !=
        s.fold_left(0int, accf)
    );
}

// Test 4: Assert empty prefix fold > full fold (prefix(0,0) should equal low, never > full)
// SHOULD FAIL
proof fn test_mutation_empty_prefix_greater() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 0int, 5int, f);
    let accfl = |acc: int, x: int| -> int { acc + f(x).len() };
    assert(s.subrange(0, 0).fold_left(5int, accfl) > s.fold_left(5int, accfl));
}

}
