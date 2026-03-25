use vstd::prelude::*;

fn main() {}

verus! {

// ===== TARGET FUNCTIONS (copied from source) =====

#[verifier::external_body]
proof fn empty_filter_implies_seq_pred_false_on_all_elements<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires s.filter(pred).len() == 0,
    ensures forall |e: A| #![auto] s.contains(e) ==> !pred(e)
{ unimplemented!() }

#[verifier::external_body]
proof fn seq_pred_false_on_all_elements_implies_empty_filter<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires forall |e: A| #![auto] s.contains(e) ==> !pred(e),
    ensures s.filter(pred).len() == 0,
{ unimplemented!() }

pub proof fn seq_pred_false_on_all_elements_is_equivalent_to_empty_filter<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures (forall |e: A| #[trigger] s.contains(e) ==> !pred(e)) <==> s.filter(pred).len() == 0,
{
    if s.len() != 0 {
        assert((forall |e: A| s.contains(e) ==> !pred(e)) ==> s.filter(pred).len() == 0) by {
            if (forall |e: A| s.contains(e) ==> !pred(e))
            {
                seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
            }
        }
        assert(s.filter(pred).len() == 0 ==> (forall |e: A| s.contains(e) ==> !pred(e))) by {
            if (s.filter(pred).len() == 0)
            {
                empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
            }
        }
    }
}

// ===== BOUNDARY TESTS =====

// Test 1: Call empty_filter_implies_... without establishing filter is empty.
// The precondition s.filter(pred).len() == 0 is not provable here.
// SHOULD FAIL
proof fn boundary_test_1_violate_empty_filter_precondition(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
{
    // We have no info about pred or filter, so the precondition cannot be proved
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
}

// Test 2: Call seq_pred_false...implies_empty_filter without establishing
// that all elements fail the predicate. The forall precondition is not provable.
// SHOULD FAIL
proof fn boundary_test_2_violate_all_false_precondition(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
{
    // We have no info about pred, so the forall precondition cannot be proved
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
}

// Test 3: Explicitly contradict the precondition - filter is known non-empty
// but we call the lemma requiring empty filter.
// SHOULD FAIL
proof fn boundary_test_3_filter_known_nonempty(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() > 0,
{
    // Filter is explicitly non-empty, directly contradicting the precondition
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
}

}
