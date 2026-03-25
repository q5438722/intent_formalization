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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Call lemma with pred1, try to derive the index-side for a STRONGER pred2.
// The equivalence is specific to the given predicate; it should NOT transfer.
// SHOULD FAIL
proof fn test_mutation_stronger_predicate() {
    let s = seq![1int, 2int, 3int];
    let pred1 = |x: int| x > 0;
    let pred2 = |x: int| x > 5;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred1);
    // We get: (∀ obj. s.contains(obj) → x>0) ⟺ (∀ i. 0≤i<3 → s[i]>0)
    // Both sides are true for pred1.
    // Mutated: try to derive that ALL elements > 5 (false: 1,2,3 are NOT > 5)
    assert(forall |i: int| 0 <= i < s.len() ==> pred2(s[i]));
}

// Test 2: Call lemma with s1, try to derive the containment-side for a DIFFERENT s2.
// The equivalence is specific to the given sequence; it should NOT transfer.
// SHOULD FAIL
proof fn test_mutation_different_sequence() {
    let s1 = seq![1int, 2int, 3int];
    let s2 = seq![1int, -1int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s1, pred);
    // We get equivalence for s1 with pred (both sides true).
    // Mutated: try to derive pred holds for all contained elements of s2
    // s2 contains -1, which does NOT satisfy pred
    assert(forall |obj: int| s2.contains(obj) ==> pred(obj));
}

// Test 3: The biconditional ensures both sides agree. Try to derive that
// the index-side holds when the containment-side is false (they must agree).
// For s = [1, -2, 3] with pred = |x| x > 0, containment-side is false
// because -2 is in s but -2 > 0 is false. Index-side must also be false.
// SHOULD FAIL
proof fn test_mutation_break_biconditional() {
    let s = seq![1int, -2int, 3int];
    let pred = |x: int| x > 0;
    true_pred_on_all_element_equal_to_pred_on_all_index(s, pred);
    // s contains -2, pred(-2) = false → containment side is false
    // By biconditional, index side must also be false
    // Mutated: try to claim the index side is true
    assert(forall |i: int| 0 <= i < s.len() ==> pred(s[i]));
}

}
