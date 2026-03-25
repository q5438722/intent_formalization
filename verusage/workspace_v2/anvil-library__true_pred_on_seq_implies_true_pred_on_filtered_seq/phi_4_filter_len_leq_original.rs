use vstd::prelude::*;

fn main() {}

verus!{

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]), 
{ unimplemented!() }

pub proof fn true_pred_on_seq_implies_true_pred_on_filtered_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool, filter_pred: spec_fn(A) -> bool)
    requires forall |e: A| s.contains(e) ==> pred(e),
    ensures forall |e: A| s.filter(filter_pred).contains(e) ==> pred(e)
{
    assert(forall |e: A| s.filter(filter_pred).contains(e) ==> pred(e)) by {
        assert(forall |e: A| s.filter(filter_pred).contains(e) ==> #[trigger] s.contains(e)) by {
            seq_filter_is_a_subset_of_original_seq(s, filter_pred);
        }
        assert(forall |e: A| s.contains(e) ==> pred(e));
    }
}



// === Entailment query ===
proof fn phi_4_filter_len_leq_original(s: Seq<int>, pred: spec_fn(int) -> bool)
    ensures
        s.filter(pred).len() <= s.len(),
    decreases s.len(),
{
    reveal(Seq::filter);
    if s.len() != 0 {
        phi_4_filter_len_leq_original(s.drop_last(), pred);
    }
}

}
