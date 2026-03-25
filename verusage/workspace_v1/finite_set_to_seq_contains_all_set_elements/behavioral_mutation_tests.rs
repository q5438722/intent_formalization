use vstd::prelude::*;

fn main() {}

verus! {

// ====== Target function signatures (copied from source) ======

#[verifier::external_body]
proof fn element_in_finite_set_exists_in_set_to_seq<A>(s: Set<A>, e: A)
    requires s.finite(), s.contains(e),
    ensures s.to_seq().contains(e),
{ unimplemented!() }

#[verifier::external_body]
proof fn element_in_seq_exists_in_original_finite_set<A>(s: Set<A>, e: A)
    requires s.finite(), s.to_seq().contains(e),
    ensures s.contains(e),
{ unimplemented!() }

pub proof fn finite_set_to_seq_contains_all_set_elements<A>(s: Set<A>)
    requires s.finite(),
    ensures forall |e: A| #[trigger] s.contains(e) <==> #[trigger] s.to_seq().contains(e)
{
    if s.len() != 0 {
        assert forall |e: A| #[trigger] s.contains(e) implies s.to_seq().contains(e) by {
            element_in_finite_set_exists_in_set_to_seq(s, e);
        }
        assert forall |e: A| #[trigger] s.to_seq().contains(e) implies s.contains(e) by {
            element_in_seq_exists_in_original_finite_set(s, e);
        }
    }
}

// ====== Behavioral Mutation Tests ======

// Test 1: Assert absent element appears in sequence (mutated output: false positive)
// SHOULD FAIL
proof fn test_mutation_absent_in_seq() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    // 42 is not in s, so should not be in s.to_seq()
    assert(s.to_seq().contains(42int));
}

// Test 2: Assert present element is NOT in sequence (mutated output: false negative)
// SHOULD FAIL
proof fn test_mutation_present_not_in_seq() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    // 1 is in s, so should be in s.to_seq(); negation should fail
    assert(!s.to_seq().contains(1int));
}

// Test 3: Assert element is in set when it is not (mutated membership)
// SHOULD FAIL
proof fn test_mutation_wrong_membership() {
    let s = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s);
    // 99 is not in s
    assert(s.contains(99int));
}

// Test 4: Assert the biconditional is an implication only (weaken ensures)
// Try to show seq contains element but set does not — contradiction with spec
// SHOULD FAIL
proof fn test_mutation_seq_has_extra() {
    let s = Set::<int>::empty().insert(10int);
    finite_set_to_seq_contains_all_set_elements(s);
    // Assert that sequence contains 20 (not in set) — should be rejected
    assert(s.to_seq().contains(20int));
    assert(!s.contains(20int));
}

// Test 5: Assert negated biconditional for a valid element
// SHOULD FAIL
proof fn test_mutation_negated_biconditional() {
    let s = Set::<int>::empty().insert(5int);
    finite_set_to_seq_contains_all_set_elements(s);
    // The biconditional should hold; asserting it doesn't should fail
    assert(!(s.contains(5int) <==> s.to_seq().contains(5int)));
}

}
