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
proof fn phi_1_fold_left_push_empty()
    ensures
        Seq::<int>::empty().push(42).fold_left(0int, |acc: int, x: int| acc + x) == 42,
{
    lemma_fold_left_push(Seq::<int>::empty(), 42, 0int, |acc: int, x: int| acc + x);
}

}
