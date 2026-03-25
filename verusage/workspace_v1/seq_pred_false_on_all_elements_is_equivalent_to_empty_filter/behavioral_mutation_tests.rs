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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Negate the postcondition of empty_filter_implies...
// When filter is empty and element is in s, the spec says !pred(e).
// We mutate to assert pred(e) — the opposite.
// SHOULD FAIL
proof fn behavioral_mutation_1_negate_postcondition(s: Seq<int>, pred: spec_fn(int) -> bool, e: int)
    requires
        s.filter(pred).len() == 0,
        s.contains(e),
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    // Postcondition gives: forall |e| s.contains(e) ==> !pred(e)
    // So !pred(e) should hold. We assert the opposite:
    assert(pred(e)); // SHOULD FAIL
}

// Test 2: Negate the postcondition of seq_pred_false...implies_empty_filter.
// When all elements fail pred, the spec says filter.len() == 0.
// We mutate to assert filter.len() > 0.
// SHOULD FAIL
proof fn behavioral_mutation_2_negate_filter_empty(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        forall |e: int| #![auto] s.contains(e) ==> !pred(e),
{
    seq_pred_false_on_all_elements_implies_empty_filter(s, pred);
    // Postcondition gives: s.filter(pred).len() == 0
    // We assert the opposite:
    assert(s.filter(pred).len() > 0); // SHOULD FAIL
}

// Test 3: Negate the biconditional from the equivalence.
// The spec ensures (forall ... <==> filter.len() == 0).
// We mutate to assert the negation of the biconditional.
// SHOULD FAIL
proof fn behavioral_mutation_3_negate_biconditional(s: Seq<int>, pred: spec_fn(int) -> bool)
{
    seq_pred_false_on_all_elements_is_equivalent_to_empty_filter(s, pred);
    // Postcondition gives the biconditional
    // We assert its negation:
    assert(
        !((forall |e: int| #[trigger] s.contains(e) ==> !pred(e)) <==> s.filter(pred).len() == 0)
    ); // SHOULD FAIL
}

}
