use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemma (copied from source) =====

pub proof fn lemma_filter_skip_rejected<A>(s: Seq<A>, pred: spec_fn(A) -> bool, i: int)
    requires
        0 <= i <= s.len(),
        forall |j| 0 <= j < i ==> !pred(s[j]),
    ensures
        s.filter(pred) == s.skip(i).filter(pred)
    decreases
        s.len()
{
    reveal(Seq::filter);
    if s.len() == 0 {
        assert(s.skip(i) =~= s);
    }
    else if i < s.len() {
        assert(s.skip(i).drop_last() =~= s.drop_last().skip(i));
        lemma_filter_skip_rejected(s.drop_last(), pred, i);
    }
    else {
        assert(s.skip(i) =~= s.drop_last().skip(i - 1));
        lemma_filter_skip_rejected(s.drop_last(), pred, i - 1);
    }
}

// ===== Boundary Tests =====
// These tests violate the preconditions (requires) of the lemma.

// SHOULD FAIL: i is negative, violates 0 <= i
proof fn test_boundary_negative_index() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, -1);
}

// SHOULD FAIL: i exceeds sequence length, violates i <= s.len()
proof fn test_boundary_i_exceeds_length() {
    let s = seq![1int, 2, 3];
    let pred = |x: int| x > 10;
    lemma_filter_skip_rejected(s, pred, 4);
}

// SHOULD FAIL: element at index 1 satisfies pred, violates forall precondition
proof fn test_boundary_middle_element_satisfies_pred() {
    let s = seq![1int, 20, 3];
    let pred = |x: int| x > 10;
    // s[1] = 20 > 10, but we skip with i = 2, so precondition violated
    lemma_filter_skip_rejected(s, pred, 2);
}

// SHOULD FAIL: first element satisfies pred, violates forall precondition
proof fn test_boundary_first_element_satisfies_pred() {
    let s = seq![100int, 2, 3];
    let pred = |x: int| x > 10;
    // s[0] = 100 > 10, skip with i = 1
    lemma_filter_skip_rejected(s, pred, 1);
}

// SHOULD FAIL: all elements before i satisfy pred, violates forall precondition
proof fn test_boundary_all_before_i_satisfy_pred() {
    let s = seq![50int, 60, 70];
    let pred = |x: int| x > 10;
    // All elements satisfy pred, skip with i = 3
    lemma_filter_skip_rejected(s, pred, 3);
}

}
