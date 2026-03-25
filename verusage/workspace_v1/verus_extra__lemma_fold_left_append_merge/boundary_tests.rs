use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus!{

// Original lemma (included so tests can call it)
pub proof fn lemma_fold_left_append_merge<A, B>(s1: Seq<A>, s2: Seq<A>, f: spec_fn(A) -> Seq<B>)
  ensures
    (s1 + s2).fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      ==
    s1.fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
      +
    s2.fold_left(Seq::empty(), |acc: Seq<B>, a: A| acc + f(a))
  decreases
    s1.len() + s2.len()
{
  let e = Seq::<B>::empty();
  let af = |acc: Seq<B>, a: A| acc + f(a);
  let fl = |s: Seq<A>| s.fold_left(e, af);
  if s2.len() == 0 {
    assert(s1 + s2 =~= s1);
    assert(fl(s1) =~= fl(s1) + e);
  } else {
    lemma_fold_left_append_merge(s1, s2.drop_last(), f);
    assert((s1 + s2).drop_last() =~= s1 + s2.drop_last());
    assert((fl(s1) + fl(s2.drop_last())) + f(s2.last()) =~= fl(s1) + (fl(s2.drop_last()) + f(s2.last())));
  }
}

// SHOULD FAIL
// Test 1: Distribution does NOT hold with non-empty initial accumulator.
// The lemma only guarantees distribution starting from Seq::empty().
// With a non-empty init, init appears once on LHS but twice on RHS.
proof fn test_boundary_nonempty_accumulator(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>, init: Seq<int>)
  requires
    init.len() > 0,
    s1.len() > 0,
    s2.len() > 0,
  ensures
    (s1 + s2).fold_left(init, |acc: Seq<int>, a: int| acc + f(a))
      ==
    s1.fold_left(init, |acc: Seq<int>, a: int| acc + f(a))
      +
    s2.fold_left(init, |acc: Seq<int>, a: int| acc + f(a))
{
}

// SHOULD FAIL
// Test 2: Fold of empty sequence should return empty, not something non-empty.
// fold_left(empty, empty, f) == empty, so its length cannot be > 0.
proof fn test_boundary_empty_fold_nonempty(f: spec_fn(int) -> Seq<int>)
  ensures
    Seq::<int>::empty().fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a)).len() > 0
{
}

// SHOULD FAIL
// Test 3: Fold of a non-empty sequence with a function that always produces
// non-empty output cannot have length 0.
proof fn test_boundary_nonempty_fold_gives_empty(s: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s.len() > 0,
    forall |x: int| #[trigger] f(x).len() > 0,
  ensures
    s.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a)).len() == 0
{
}

}
