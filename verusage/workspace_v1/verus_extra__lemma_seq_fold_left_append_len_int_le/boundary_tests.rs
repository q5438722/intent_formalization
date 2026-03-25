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

// === BOUNDARY TESTS ===
// These tests violate preconditions and should fail verification.

// Test 1: Negative index i = -1 violates requires 0 <= i
// SHOULD FAIL
proof fn test_boundary_negative_index() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, -1int, 0int, f);
}

// Test 2: Index i = s.len() + 1 violates requires i <= s.len()
// SHOULD FAIL
proof fn test_boundary_index_exceeds_len() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 4int, 0int, f);
}

// Test 3: Negative low = -1 violates requires 0 <= low
// SHOULD FAIL
proof fn test_boundary_negative_low() {
    let s: Seq<int> = seq![1int, 2int, 3int];
    let f = |x: int| -> Seq<int> { seq![x] };
    lemma_seq_fold_left_append_len_int_le::<int, int>(s, 1int, -1int, f);
}

// Test 4: Empty sequence violates requires s.len() > 0 for sum_right
// SHOULD FAIL
proof fn test_boundary_empty_seq_sum_right() {
    let s: Seq<int> = Seq::empty();
    let f = |x: int| -> int { x };
    lemma_seq_fold_left_sum_right::<int>(s, 0int, f);
}

}
