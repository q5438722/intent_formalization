use vstd::prelude::*;

fn main() {}

verus! {

// ===== Original specifications (copied for self-contained compilation) =====

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

// ===== Boundary Tests =====

// --- Boundary Test 1: Violate precondition ---
// pred does NOT hold for all elements of s (e.g., 1 > 2 is false).
// Calling the lemma without satisfying requires should be rejected.
// SHOULD FAIL
proof fn test_boundary_precondition_violation() {
    let s = seq![1int, 2int, 3int];
    true_pred_on_seq_implies_true_pred_on_filtered_seq::<int>(
        s,
        |x: int| x > 2,   // 1 and 2 do NOT satisfy this
        |x: int| x > 0,
    );
}

// --- Boundary Test 2: Assert non-empty filter result from empty sequence ---
// An empty sequence filtered by any predicate should still be empty.
// SHOULD FAIL
proof fn test_boundary_empty_seq_nonempty_filter() {
    let s = Seq::<int>::empty();
    seq_filter_is_a_subset_of_original_seq::<int>(s, |x: int| true);
    assert(s.filter(|x: int| true).len() > 0);
}

// --- Boundary Test 3: Assert containment for out-of-bounds index ---
// The axiom only guarantees containment for indices 0 <= i < filter.len().
// Asserting containment for index far beyond length should be rejected.
// SHOULD FAIL
proof fn test_boundary_out_of_bounds_index() {
    let s = seq![1int, 2int, 3int];
    let filter_pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq::<int>(s, filter_pred);
    assert(s.contains(s.filter(filter_pred)[100]));
}

}
