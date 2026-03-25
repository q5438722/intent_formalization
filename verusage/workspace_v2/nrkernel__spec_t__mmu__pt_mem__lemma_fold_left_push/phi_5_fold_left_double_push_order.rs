use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/pt_mem.rs
proof fn lemma_fold_left_push<A,B>(s: Seq<A>, a: A, b: B, f: spec_fn(B, A) -> B)
    ensures s.push(a).fold_left(b, f) == f(s.fold_left(b, f), a)
{
    assert(s.push(a).drop_last() == s);
}




// === Entailment query ===
proof fn phi_5_fold_left_double_push_order(s: Seq<int>, a1: int, a2: int, init: Seq<int>)
    ensures
        s.push(a1).push(a2).fold_left(init, |acc: Seq<int>, x: int| acc.push(x))
          == s.push(a1).fold_left(init, |acc: Seq<int>, x: int| acc.push(x)).push(a2),
{
    lemma_fold_left_push(s.push(a1), a2, init, |acc: Seq<int>, x: int| acc.push(x));
}

}
