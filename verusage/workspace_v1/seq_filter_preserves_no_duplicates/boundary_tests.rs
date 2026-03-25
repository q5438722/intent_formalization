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

// ========== BOUNDARY TESTS ==========

// Test 1: Adjacent duplicates violate no_duplicates precondition
// Calling seq_filter_preserves_no_duplicates on a sequence with adjacent duplicates
// should be rejected because s.no_duplicates() is false.
// SHOULD FAIL
proof fn test_boundary_adjacent_duplicates()
{
    let s: Seq<int> = seq![1int, 1];
    let pred = |x: int| x > 0;
    seq_filter_preserves_no_duplicates(s, pred);
}

// Test 2: Distant duplicates violate no_duplicates precondition
// Even when duplicate elements are separated by other elements,
// no_duplicates still fails and the precondition should be rejected.
// SHOULD FAIL
proof fn test_boundary_distant_duplicates()
{
    let s: Seq<int> = seq![1int, 2, 3, 1];
    let pred = |x: int| x >= 0;
    seq_filter_preserves_no_duplicates(s, pred);
}

// Test 3: All identical elements violate no_duplicates precondition
// A sequence of all identical elements is maximally duplicated.
// SHOULD FAIL
proof fn test_boundary_all_identical()
{
    let s: Seq<int> = seq![7int, 7, 7];
    let pred = |x: int| true;
    seq_filter_preserves_no_duplicates(s, pred);
}

}
