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
proof fn phi_3_fold_left_push_ignores_order(a: int, b: int)
    requires
        a != b,
    ensures
        seq![a].push(b).fold_left(0int, |acc: int, x: int| acc - x)
            != seq![b].push(a).fold_left(0int, |acc: int, x: int| acc - x),
{
    lemma_fold_left_push(seq![a], b, 0int, |acc: int, x: int| acc - x);
    lemma_fold_left_push(seq![b], a, 0int, |acc: int, x: int| acc - x);
}

}
