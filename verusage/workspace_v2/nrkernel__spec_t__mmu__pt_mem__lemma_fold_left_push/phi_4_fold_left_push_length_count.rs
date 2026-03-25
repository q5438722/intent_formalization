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
proof fn phi_4_fold_left_push_length_count(s: Seq<int>, a: int)
    ensures
        s.push(a).fold_left(0int, |acc: int, _x: int| acc + 1) == s.fold_left(0int, |acc: int, _x: int| acc + 1) + 1,
{
    lemma_fold_left_push(s, a, 0int, |acc: int, _x: int| acc + 1);
}

}
