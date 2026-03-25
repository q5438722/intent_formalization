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

// Test 1: Violate precondition 1 — different sequence lengths
// SHOULD FAIL
proof fn test_boundary_different_lengths() {
    let s1 = Seq::<int>::new(3, |i: int| i);
    let s2 = Seq::<int>::new(2, |i: int| i);
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| a == b,
        0int,
        |acc: int, x: int| (acc + x),
    );
}

// Test 2: Violate precondition 2 — elements not equivalent under eq
// SHOULD FAIL
proof fn test_boundary_non_equiv_elements() {
    let s1 = Seq::<int>::new(3, |i: int| i);
    let s2 = Seq::<int>::new(3, |i: int| (i + 10));
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| a == b,
        0int,
        |acc: int, x: int| (acc + x),
    );
}

// Test 3: Violate precondition 3 — f does not respect eq
// eq is trivially true for all pairs, but f(b,a1) != f(b,a2) for distinct a1,a2
// SHOULD FAIL
proof fn test_boundary_f_not_respecting_eq() {
    let s1 = Seq::<int>::new(2, |i: int| i);
    let s2 = Seq::<int>::new(2, |i: int| (i + 10));
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| true,
        0int,
        |acc: int, x: int| (acc + x),
    );
}

}
