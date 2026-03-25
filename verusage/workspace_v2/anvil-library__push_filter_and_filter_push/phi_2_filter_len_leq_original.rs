use vstd::prelude::*;

fn main() {}

verus!{


pub proof fn push_filter_and_filter_push<A>(s: Seq<A>, pred: spec_fn(A) -> bool, e: A)
    ensures
        pred(e) ==> s.push(e).filter(pred) == s.filter(pred).push(e),
        !pred(e) ==> s.push(e).filter(pred) == s.filter(pred),
{
    reveal(Seq::filter);
    assert(s.push(e).drop_last() == s);
}



// === Entailment query ===
proof fn phi_2_filter_len_leq_original(s: Seq<int>, pred: spec_fn(int) -> bool)
    ensures
        s.filter(pred).len() <= s.len(),
    decreases s.len(),
{
    reveal(Seq::filter);
    if s.len() != 0 {
        phi_2_filter_len_leq_original(s.drop_last(), pred);
    }
}

}
