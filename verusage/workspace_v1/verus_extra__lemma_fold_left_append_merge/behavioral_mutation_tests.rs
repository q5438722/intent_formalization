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
// Test 1: Mutated output — reversed concatenation order.
// The lemma guarantees fold(s1+s2) == fold(s1) + fold(s2).
// Claiming fold(s1+s2) == fold(s2) + fold(s1) requires commutativity
// of sequence concatenation, which does not hold in general.
proof fn test_mutation_reversed_order(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s1.len() > 0,
    s2.len() > 0,
  ensures
    (s1 + s2).fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      ==
    s2.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      +
    s1.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
{
  lemma_fold_left_append_merge(s1, s2, f);
}

// SHOULD FAIL
// Test 2: Mutated output — s2 contribution dropped.
// The lemma gives fold(s1+s2) == fold(s1) + fold(s2).
// Claiming fold(s1+s2) == fold(s1) ignores s2's contribution entirely.
proof fn test_mutation_missing_s2(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s2.len() > 0,
    forall |x: int| #[trigger] f(x).len() > 0,
  ensures
    (s1 + s2).fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      ==
    s1.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
{
  lemma_fold_left_append_merge(s1, s2, f);
}

// SHOULD FAIL
// Test 3: Off-by-one mutation — first element of s1 dropped.
// Claiming fold(s1+s2) == fold(s1[1..]) + fold(s2) skips s1's first element.
proof fn test_mutation_drop_first_s1(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s1.len() > 1,
    forall |x: int| #[trigger] f(x).len() > 0,
  ensures
    (s1 + s2).fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      ==
    s1.subrange(1, s1.len() as int).fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      +
    s2.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
{
  lemma_fold_left_append_merge(s1, s2, f);
}

}
