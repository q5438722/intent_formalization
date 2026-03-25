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
proof fn phi_5_empty_filter_empty(pred: spec_fn(int) -> bool)
    ensures
        Seq::<int>::empty().filter(pred).len() == 0,
{
    reveal(Seq::filter);
}

}
