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
proof fn phi_3_fold_left_push_commute_with_const(s: Seq<int>, a: int, init: int)
    ensures
        s.push(a).fold_left(init, |acc: int, _x: int| acc) == init,
{
    lemma_fold_left_push(s, a, init, |acc: int, _x: int| acc);
}

}
