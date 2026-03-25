use vstd::prelude::*;

fn main() {}

verus!{

pub proof fn true_pred_on_all_element_equal_to_pred_on_all_index<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        (forall |obj: A| #[trigger] s.contains(obj) ==> pred(obj)) <==> (forall |i: int| 0 <= i < s.len() ==> pred(s[i]))
{
    if s.len() != 0 {
        assert((forall |i: int| 0 <= i < s.len() ==> pred(s[i])) ==> (forall |obj: A| s.contains(obj) ==> pred(obj)));
        assert((forall |obj: A| s.contains(obj) ==> pred(obj)) ==> (forall |i: int| 0 <= i < s.len() ==> pred(s[i]))) by {
            if (forall |obj: A| s.contains(obj) ==> pred(obj)) {
                assert(forall |i: int| 0 <= i < s.len() ==> pred(s[i])) by {
                    assert(forall |i: int| 0 <= i < s.len() ==> s.contains(#[trigger] s[i]) ==> pred(s[i]));
                }
            }
        }
    }
}



// === Entailment query ===
proof fn phi_5_negated_pred_false_element_not_in_seq(s: Seq<int>, pred: spec_fn(int) -> bool, e: int)
    requires
        forall |i: int| 0 <= i < s.len() ==> pred(s[i]),
        !pred(e),
    ensures
        !s.contains(e),
{
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
}

}
