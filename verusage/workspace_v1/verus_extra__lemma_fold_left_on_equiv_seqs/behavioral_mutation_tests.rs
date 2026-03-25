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

// Test 1: Negate postcondition — assert fold results are NOT equal after valid call
// SHOULD FAIL
proof fn test_mutation_negate_postcondition() {
    let s1 = Seq::<int>::new(3, |i: int| (i + 1));
    let s2 = Seq::<int>::new(3, |i: int| (i + 1));
    let f = |acc: int, x: int| (acc + x);
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| a == b,
        0int,
        f,
    );
    assert(s1.fold_left(0int, f) != s2.fold_left(0int, f));
}

// Test 2: Off-by-one mutation — assert fold(s1) == fold(s2) + 1
// SHOULD FAIL
proof fn test_mutation_off_by_one() {
    let s1 = Seq::<int>::new(2, |i: int| (i + 1));
    let s2 = Seq::<int>::new(2, |i: int| (i + 1));
    let f = |acc: int, x: int| (acc + x);
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| a == b,
        0int,
        f,
    );
    assert(s1.fold_left(0int, f) == s2.fold_left(0int, f) + 1);
}

// Test 3: Different init values — claim fold(s1,0,f) == fold(s2,100,f) which is wrong
// SHOULD FAIL
proof fn test_mutation_different_init() {
    let s1 = Seq::<int>::new(2, |i: int| (i + 1));
    let s2 = Seq::<int>::new(2, |i: int| (i + 1));
    let f = |acc: int, x: int| (acc + x);
    lemma_fold_left_on_equiv_seqs::<int, int>(
        s1, s2,
        |a: int, b: int| a == b,
        0int,
        f,
    );
    assert(s1.fold_left(0int, f) == s2.fold_left(100int, f));
}

}
