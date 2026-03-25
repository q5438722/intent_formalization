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

// ===== LOGICAL TESTS =====

// Test 1: Over-generalization — try to derive that s is empty from filter being empty.
// The spec says filter.len() == 0 implies no element satisfies pred,
// but it does NOT imply the sequence itself is empty.
// SHOULD FAIL
proof fn logical_test_1_empty_filter_implies_empty_seq(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    // We know: no element in s satisfies pred
    // But we cannot conclude that s is empty:
    assert(s.len() == 0); // SHOULD FAIL
}

// Test 2: Scope escape — try to derive !pred(e) for an element NOT known to be in s.
// The spec only says !pred(e) for elements contained in s.
// An arbitrary e might not be in s, so !pred(e) is not guaranteed.
// SHOULD FAIL
proof fn logical_test_2_pred_false_outside_seq(s: Seq<int>, pred: spec_fn(int) -> bool, e: int)
    requires
        s.filter(pred).len() == 0,
        !s.contains(e),
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    // We know: forall |x| s.contains(x) ==> !pred(x)
    // e is NOT in s, so we cannot conclude anything about pred(e)
    assert(!pred(e)); // SHOULD FAIL
}

// Test 3: Structural uniqueness — two sequences with empty filters are not necessarily equal.
// The spec relates filter length to element predicates, not sequence identity.
// SHOULD FAIL
proof fn logical_test_3_structural_uniqueness(s: Seq<int>, t: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.filter(pred).len() == 0,
        t.filter(pred).len() == 0,
{
    empty_filter_implies_seq_pred_false_on_all_elements(s, pred);
    empty_filter_implies_seq_pred_false_on_all_elements(t, pred);
    // Both sequences have empty filters, but they need not be equal
    assert(s =~= t); // SHOULD FAIL
}

// Test 4: Incomplete reasoning — use the biconditional but try to conclude
// a specific side without knowing either direction.
// The equivalence doesn't tell us WHICH side is true, only that they agree.
// SHOULD FAIL
proof fn logical_test_4_equivalence_does_not_decide(s: Seq<int>, pred: spec_fn(int) -> bool)
    requires
        s.len() > 0,
{
    seq_pred_false_on_all_elements_is_equivalent_to_empty_filter(s, pred);
    // We know the biconditional, but not which direction holds.
    // Try to conclude filter is empty (not guaranteed):
    assert(s.filter(pred).len() == 0); // SHOULD FAIL
}

}
