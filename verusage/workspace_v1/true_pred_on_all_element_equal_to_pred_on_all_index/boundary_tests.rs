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

// ===== BOUNDARY TESTS =====

// Test 1: Empty sequence makes both sides of the biconditional vacuously true.
// Should NOT let us derive that pred holds universally for all ints.
// SHOULD FAIL
proof fn test_boundary_empty_seq_universal_pred() {
    let s: Seq<int> = Seq::<int>::empty();
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // For empty seq: (∀ obj. ∅.contains(obj) → pred(obj)) is vacuously true
    // But pred does NOT hold for all ints (e.g., -1)
    assert(forall |x: int| #[trigger] pred(x));
}

// Test 2: The spec says s.contains(obj) → pred(obj), NOT the reverse.
// pred(obj) should NOT imply s.contains(obj).
// SHOULD FAIL
proof fn test_boundary_reverse_containment_implication() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // Spec gives: ∀ obj. s.contains(obj) → pred(obj)
    // Try the reverse: ∀ x. pred(x) → s.contains(x)
    // 100 > 0 but 100 is NOT in seq![1,2,3]
    assert(forall |x: int| pred(x) ==> #[trigger] s.contains(x));
}

// Test 3: With an always-false predicate on a non-empty sequence,
// both sides of the biconditional are false. The spec should NOT
// allow deriving that either side is true.
// SHOULD FAIL
proof fn test_boundary_false_pred_nonempty_seq() {
    let s = seq![1int, 2int, 3int];
    let pred = |x: int| false;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // pred is always false, s is non-empty → both sides are false
    // Biconditional (false ⟺ false) is true, but neither side is true
    // Should NOT derive the index-side is true
    assert(forall |i: int| 0 <= i < s.len() ==> pred(s[i]));
}

}
