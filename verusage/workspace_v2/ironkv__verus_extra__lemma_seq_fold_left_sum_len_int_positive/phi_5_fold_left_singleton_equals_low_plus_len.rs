use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: seq_lib_v.rs
pub proof fn lemma_seq_fold_left_sum_len_int_positive<A, B>(s: Seq<A>, low: nat, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(low as int, |acc: int, x: A| acc + f(x).len()) >= 0,
  decreases s.len(),
{
  s.lemma_fold_left_alt(low as int, |acc: int, x: A| acc + f(x).len());
  if s.len() != 0 {
    lemma_seq_fold_left_sum_len_int_positive::<A, B>(s.subrange(1, s.len() as int), low + f(s[0]).len(), f);
    s.subrange(1, s.len() as int).lemma_fold_left_alt(low + f(s[0]).len() as int, |acc: int, x: A| acc + f(x).len());
  }
}




// === Entailment query ===
proof fn phi_5_fold_left_singleton_equals_low_plus_len<A, B>(a: A, low: nat, f: spec_fn(A) -> Seq<B>)
    ensures
        seq![a].fold_left(low as int, |acc: int, x: A| acc + f(x).len()) == low + f(a).len(),
{
    lemma_seq_fold_left_sum_len_int_positive::<A, B>(seq![a], low, f);
}

}
