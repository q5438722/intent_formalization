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
proof fn phi_1_filter_preserves_order(s: Seq<int>, pred: spec_fn(int) -> bool, i: int, j: int)
    requires
        0 <= i < s.filter(pred).len(),
        0 <= j < s.filter(pred).len(),
        i < j,
    ensures
        exists |ii: int, jj: int| 0 <= ii < jj < s.len() && s[ii] == s.filter(pred)[i] && s[jj] == s.filter(pred)[j],
{
}

}
