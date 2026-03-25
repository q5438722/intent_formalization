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
proof fn phi_4_fold_left_push_concat_equivalence(s: Seq<int>, a: int, b: int)
    ensures
        s.push(a).push(b).fold_left(0int, |acc: int, x: int| acc + x)
            == s.fold_left(0int, |acc: int, x: int| acc + x) + a + b,
{
    lemma_fold_left_push(s.push(a), b, 0int, |acc: int, x: int| acc + x);
    lemma_fold_left_push(s, a, 0int, |acc: int, x: int| acc + x);
}

}
