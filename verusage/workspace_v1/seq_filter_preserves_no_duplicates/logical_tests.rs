use vstd::prelude::*;

fn main() {}

verus! {

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{unimplemented!()}

pub proof fn seq_filter_preserves_no_duplicates<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    requires s.no_duplicates(),
    ensures s.filter(pred).no_duplicates()
    decreases s.len()
{
    reveal(Seq::filter);
    if s.len() != 0 {
        seq_filter_preserves_no_duplicates(s.drop_last(), pred);
        if pred(s.last()) {
            seq_filter_is_a_subset_of_original_seq(s.drop_last(), pred);
        }
    }
}

// ========== LOGICAL TESTS ==========

// Test 1: Assert filter preserves no_duplicates WITHOUT the precondition.
// The theorem requires s.no_duplicates(), but here s has duplicates.
// We skip calling the proof function and assert the conclusion directly.
// This tests whether unintended reasoning can derive no_duplicates without the precondition.
// SHOULD FAIL
proof fn test_logical_no_dup_without_precondition()
{
    let s: Seq<int> = seq![1int, 1, 2];
    let pred = |x: int| true;
    // Intentionally NOT calling seq_filter_preserves_no_duplicates
    assert(s.filter(pred).no_duplicates());
}

// Test 2: Soundness check — try to derive `false` from the axioms.
// If the external_body axiom is unsound (its ensures are contradictory),
// calling it would introduce `false` into the proof context, allowing anything.
// This tests that the axiom does not make the logic inconsistent.
// SHOULD FAIL
proof fn test_logical_axiom_soundness()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq(s, pred);
    seq_filter_preserves_no_duplicates(s, pred);
    assert(false);
}

// Test 3: Stronger property — assert filter STRICTLY decreases length.
// The spec does not guarantee any length relationship.
// With pred = |x| true, filter should preserve all elements, so length should NOT decrease.
// This tests whether unintended stronger inequalities can be derived.
// SHOULD FAIL
proof fn test_logical_filter_strict_length_decrease()
{
    let s: Seq<int> = seq![1int, 2, 3];
    let pred = |x: int| true;
    seq_filter_preserves_no_duplicates(s, pred);
    assert(s.filter(pred).len() < s.len());
}

}
