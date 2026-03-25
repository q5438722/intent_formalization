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
proof fn phi_5_fold_left_push_closure_mismatch()
    ensures
        seq![1int].push(2).fold_left(10int, |acc: int, x: int| x)
            == (|acc: int, x: int| x)(seq![1int].fold_left(10int, |acc: int, x: int| x), 2),
{
    lemma_fold_left_push(seq![1int], 2, 10int, |acc: int, x: int| x);
}

}
