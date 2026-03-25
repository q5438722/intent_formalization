use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: seq_lib_v.rs
pub proof fn lemma_seq_fold_left_sum_le<A>(s: Seq<A>, init: int, high: int, f: spec_fn(A) -> int)
  requires
    forall |i:int| 0 <= i < s.len() ==> f(s[i]) <= high,
  ensures
    s.fold_left(init, |acc: int, x: A| acc + f(x)) <= init + s.len() * high,
  decreases s.len(),
{
  if s.len() != 0 {
    lemma_seq_fold_left_sum_le(s.drop_last(), init, high, f);
    assert(init + (s.len() - 1) * high + high <= init + s.len() * high) by (nonlinear_arith);
  }
}




// === Entailment query ===
proof fn phi_4_fold_left_sum_le_high_much_larger<A>(s: Seq<A>, init: int, f: spec_fn(A) -> int)
    requires
        s.len() > 0,
        forall |i: int| 0 <= i < s.len() ==> f(s[i]) <= 0,
    ensures
        s.fold_left(init, |acc: int, x: A| acc + f(x)) <= init + s.len() * 1000000,
{
    lemma_seq_fold_left_sum_le(s, init, 1000000, f);
}

}
