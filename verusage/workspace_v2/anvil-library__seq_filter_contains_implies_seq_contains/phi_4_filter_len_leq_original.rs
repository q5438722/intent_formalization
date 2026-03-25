use vstd::prelude::*;

fn main() {}

verus!{

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]), 
{ unimplemented!() }

pub proof fn seq_filter_contains_implies_seq_contains<A>(s: Seq<A>, pred: spec_fn(A) -> bool, elt: A)
    requires s.filter(pred).contains(elt),
    ensures s.contains(elt)
{
    seq_filter_is_a_subset_of_original_seq(s, pred);
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
