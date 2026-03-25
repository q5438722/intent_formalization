use vstd::prelude::*;

fn main() {}

verus! {

// ===== Target function (copied from source) =====

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

// ===== LOGICAL TESTS =====

// Test 1: If pred holds on all elements of s, it should NOT transfer to
// a different sequence s.push(x) for arbitrary x.
// SHOULD FAIL
proof fn test_logical_no_cross_sequence_transfer() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // pred holds on all of s (1>0, 2>0, 3>0)
    let s2 = s.push(-1int);
    // Should NOT derive pred holds on all of s2 (since -1 is in s2)
    assert(forall |obj: int| s2.contains(obj) ==> pred(obj));
}

// Test 2: Two sequences where pred holds on all elements of both
// should NOT be derivable as equal. Sequences with same predicate
// behavior can have completely different elements.
// SHOULD FAIL
proof fn test_logical_no_sequence_equality_from_pred() {
    let s1 = seq![1int, 2int];
    let s2 = seq![3int, 4int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s1, pred);
    true_pred_on_all_element_equal_to_pred_on_all_index(s2, pred);
    // Both s1 and s2 satisfy: ∀ obj. s.contains(obj) → pred(obj)
    // But they are clearly different sequences
    assert(s1 =~= s2);
}

// Test 3: Knowing that pred holds on all elements of s should NOT imply
// pred holds for elements NOT in s. The containment guard is essential.
// SHOULD FAIL
proof fn test_logical_pred_not_universally_derivable() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // We know: ∀ obj. s.contains(obj) → pred(obj) (all in s are > 0)
    // But pred is NOT true for all ints
    // Specifically, pred(-1) should NOT be derivable
    assert(pred(-1int));
}

// Test 4: Calling the lemma for two different predicates on the same sequence
// should NOT allow deriving an implication between the predicates.
// SHOULD FAIL
proof fn test_logical_no_pred_implication_across_calls() {
    let s = seq![1int, 2int, 3int];
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x < 10;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred1);
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred2);
    // Both predicates hold on all elements of s
    // Should NOT derive: ∀ x. pred1(x) → pred2(x) (i.e., x > 0 → x < 10)
    // Counterexample: x = 100 satisfies pred1 but not pred2
    assert(forall |x: int| #[trigger] pred1(x) ==> pred2(x));
}

}
