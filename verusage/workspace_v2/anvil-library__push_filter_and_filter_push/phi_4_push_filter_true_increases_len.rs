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
proof fn phi_4_push_filter_true_increases_len(s: Seq<int>, pred: spec_fn(int) -> bool, e: int)
    requires
        pred(e),
    ensures
        s.push(e).filter(pred).len() == s.filter(pred).len() + 1,
{
    push_filter_and_filter_push(s, pred, e);
}

}
