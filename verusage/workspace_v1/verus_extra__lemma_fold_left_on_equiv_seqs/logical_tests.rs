use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

pub proof fn lemma_fold_left_on_equiv_seqs<A, B>(s1: Seq<A>, s2: Seq<A>, eq: spec_fn(A, A) -> bool, init: B, f: spec_fn(B, A) -> B)
    requires
      s1.len() == s2.len(),
      (forall |i: int| 0 <= i < s1.len() ==> eq(s1[i], s2[i])),
      (forall |b: B, a1: A, a2: A| #[trigger] eq(a1, a2) ==> #[trigger] f(b, a1) == f(b, a2)),
    ensures
      s1.fold_left(init, f) == s2.fold_left(init, f)
    decreases s1.len(),
{
  reveal(Seq::fold_left);
  if s1.len() != 0 {
    lemma_fold_left_on_equiv_seqs(s1.drop_last(), s2.drop_last(), eq, init, f);
  }
}

// Test 1: Stronger conclusion — claim s1 =~= s2 from weak equivalence
// eq-equivalence does NOT imply extensional (actual) equality
// SHOULD FAIL
proof fn test_logical_extensional_equality() {
    let s1 = Seq::<int>::new(2, |i: int| (i * 2));
    let s2 = Seq::<int>::new(2, |i: int| (i * 2 + 4));
    let eq_fn = |a: int, b: int| (a % 2) == (b % 2);
    let f = |acc: int, x: int| acc + (x % 2);
    lemma_fold_left_on_equiv_seqs::<int, int>(s1, s2, eq_fn, 0int, f);
    // Fold results are equal, but the sequences themselves are not
    assert(s1 =~= s2);
}

// Test 2: Order independence — fold_left is NOT commutative for non-commutative f
// SHOULD FAIL
proof fn test_logical_order_independence() {
    let s1 = Seq::<int>::new(3, |i: int| (i + 1));
    let s2 = Seq::<int>::new(3, |i: int| (3 - i));
    let f = |acc: int, x: int| (acc * 10 + x);
    // fold_left([1,2,3], 0, f) = 123
    // fold_left([3,2,1], 0, f) = 321
    // These are NOT equal
    assert(s1.fold_left(0int, f) == s2.fold_left(0int, f));
}

// Test 3: Cross-function misuse — fold equality for f does NOT extend to arbitrary g
// SHOULD FAIL
proof fn test_logical_cross_function_misuse() {
    let s1 = Seq::<int>::new(2, |i: int| (i * 2));
    let s2 = Seq::<int>::new(2, |i: int| (i * 2 + 4));
    let eq_fn = |a: int, b: int| (a % 2) == (b % 2);
    let f = |acc: int, x: int| acc + (x % 2);
    let g = |acc: int, x: int| (acc + x);
    // Call lemma for f (which respects eq_fn)
    lemma_fold_left_on_equiv_seqs::<int, int>(s1, s2, eq_fn, 0int, f);
    // Try to claim fold equality for g (which does NOT respect eq_fn)
    // fold([0,2], 0, g) = 2, fold([4,6], 0, g) = 10 — NOT equal
    assert(s1.fold_left(0int, g) == s2.fold_left(0int, g));
}

}
