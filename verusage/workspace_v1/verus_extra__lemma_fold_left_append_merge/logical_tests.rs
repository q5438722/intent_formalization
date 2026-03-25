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
// Test 1: Commutativity — fold results are NOT commutative under concatenation.
// fold(s1) + fold(s2) != fold(s2) + fold(s1) in general,
// because sequence concatenation is order-dependent.
proof fn test_logical_commutativity(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s1.len() > 0,
    s2.len() > 0,
  ensures
    s1.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      +
    s2.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      ==
    s2.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      +
    s1.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
{
}

// SHOULD FAIL
// Test 2: Length preservation — fold result length does NOT equal input sequence length.
// The fold result length depends on f's output sizes, not just the number of elements.
proof fn test_logical_length_preservation(s: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s.len() > 0,
  ensures
    s.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a)).len() == s.len()
{
}

// SHOULD FAIL
// Test 3: Injectivity — equal fold results do NOT imply equal input sequences.
// Different sequences can produce the same fold output (e.g., via a constant f).
proof fn test_logical_injectivity(s1: Seq<int>, s2: Seq<int>, f: spec_fn(int) -> Seq<int>)
  requires
    s1.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a))
      ==
    s2.fold_left(Seq::<int>::empty(), |acc: Seq<int>, a: int| acc + f(a)),
  ensures
    s1 =~= s2
{
}

}
