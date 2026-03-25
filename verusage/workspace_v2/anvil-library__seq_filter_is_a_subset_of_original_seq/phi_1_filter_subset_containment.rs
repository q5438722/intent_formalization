use vstd::prelude::*;

fn main() {}

verus!{


pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]), // 2nd form
    decreases s.len()
{
    reveal(Seq::filter);
    if s.filter(pred).len() != 0 {
        let subseq = s.drop_last();
        seq_filter_is_a_subset_of_original_seq(subseq, pred);
        assert(forall |i: int| 0 <= i < subseq.filter(pred).len() ==> subseq.contains(#[trigger] subseq.filter(pred)[i]));
        // assert(forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]));
        // assert(forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e));
    }
}



// === Entailment query ===
proof fn phi_1_filter_subset_containment(s: Seq<int>, pred: spec_fn(int) -> bool, e: int)
    requires
        s.filter(pred).contains(e),
    ensures
        s.contains(e),
{
    seq_filter_is_a_subset_of_original_seq(s, pred);
}

}
