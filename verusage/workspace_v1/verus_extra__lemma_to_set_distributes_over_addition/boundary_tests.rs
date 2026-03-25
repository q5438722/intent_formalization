use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Original function (copied from target) =====

#[verifier::spinoff_prover]
pub proof fn lemma_to_set_distributes_over_addition<A>(s: Seq<A>, t: Seq<A>)
    ensures (s+t).to_set() == s.to_set() + t.to_set()
{
    let left = (s+t).to_set();
    let right = s.to_set() + t.to_set();
    assert forall |x| right.contains(x) implies left.contains(x) by {
        assert(s.to_set()+t.to_set() == s.to_set().union(t.to_set()));
        if s.to_set().contains(x) {
            let si = choose |si| 0<=si<s.len() && s[si] == x;
            assert((s+t)[si] == x);
        } else {
            let ti = choose |ti| 0<=ti<t.len() && t[ti] == x;
            assert((s+t)[s.len() + ti] == x);
        }
    }
    assert_sets_equal!(left, right);
}

// ========== BOUNDARY TESTS ==========
// The spec has NO preconditions (requires), so it claims to hold for ALL inputs.
// These tests probe edge-case inputs and assert false properties about the result.

// Test 1: Empty + empty → result should be the empty set.
// Asserting it contains an element should be rejected.
// SHOULD FAIL
proof fn test_boundary_empty_concat_has_element() {
    let s = Seq::<int>::empty();
    let t = Seq::<int>::empty();
    lemma_to_set_distributes_over_addition(s, t);
    // Both sequences are empty, so (s+t).to_set() is empty.
    // Claiming it contains 0 is false.
    assert((s + t).to_set().contains(0int));
}

// Test 2: Non-empty + empty → element from s MUST be in result.
// Asserting it is NOT in the result should be rejected.
// SHOULD FAIL
proof fn test_boundary_nonempty_element_excluded() {
    let s = seq![1int, 2int, 3int];
    let t = Seq::<int>::empty();
    lemma_to_set_distributes_over_addition(s, t);
    // 1 is in s, so s.to_set().contains(1). By the lemma, (s+t).to_set() == s.to_set() + t.to_set().
    // Thus (s+t).to_set() must contain 1. Claiming otherwise should fail.
    assert(!(s + t).to_set().contains(1int));
}

// Test 3: Two concrete sequences → assert phantom element in result.
// An element not in either sequence should NOT appear in the union.
// SHOULD FAIL
proof fn test_boundary_phantom_element() {
    let s = seq![1int, 2int];
    let t = seq![3int, 4int];
    lemma_to_set_distributes_over_addition(s, t);
    // 99 is in neither s nor t, so it cannot be in (s+t).to_set().
    assert((s + t).to_set().contains(99int));
}

}
