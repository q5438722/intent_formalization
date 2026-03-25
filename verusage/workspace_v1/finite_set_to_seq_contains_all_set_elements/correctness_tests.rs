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

// ========================================================================
// BOUNDARY TESTS — violate preconditions, check invalid inputs rejected
// ========================================================================

// Test B1: Call main lemma on infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_infinite_set() {
    let s = Set::<int>::new(|x: int| true);
    finite_set_to_seq_contains_all_set_elements(s);
}

// Test B2: Call forward axiom with element not in set (violates requires s.contains(e))
// SHOULD FAIL
proof fn test_boundary_element_not_in_set() {
    let s = Set::<int>::empty();
    element_in_finite_set_exists_in_set_to_seq(s, 42int);
}

// Test B3: Call reverse axiom on infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_reverse_non_finite() {
    let s = Set::<int>::new(|x: int| true);
    element_in_seq_exists_in_original_finite_set(s, 0int);
}

// Test B4: Call forward axiom on infinite set (violates requires s.finite())
// SHOULD FAIL
proof fn test_boundary_forward_non_finite() {
    let s = Set::<int>::new(|x: int| true);
    element_in_finite_set_exists_in_set_to_seq(s, 0int);
}

// Test B5: Call main lemma on infinite set and try to derive membership
// SHOULD FAIL
proof fn test_boundary_derive_from_infinite() {
    let s = Set::<int>::new(|x: int| x >= 0);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().contains(1int));
}

// ========================================================================
// BEHAVIORAL MUTATION TESTS — mutate expected outputs, check spec rejects
// ========================================================================

// Test M1: Assert absent element appears in sequence (false positive)
// SHOULD FAIL
proof fn test_mutation_absent_in_seq() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().contains(42int));
}

// Test M2: Assert present element is NOT in sequence (false negative)
// SHOULD FAIL
proof fn test_mutation_present_not_in_seq() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(!s.to_seq().contains(1int));
}

// Test M3: Assert wrong set membership
// SHOULD FAIL
proof fn test_mutation_wrong_membership() {
    let s = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.contains(99int));
}

// Test M4: Assert seq contains element not in set
// SHOULD FAIL
proof fn test_mutation_seq_has_extra() {
    let s = Set::<int>::empty().insert(10int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().contains(20int));
    assert(!s.contains(20int));
}

// Test M5: Assert negated biconditional for valid element
// SHOULD FAIL
proof fn test_mutation_negated_biconditional() {
    let s = Set::<int>::empty().insert(5int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(!(s.contains(5int) <==> s.to_seq().contains(5int)));
}

// ========================================================================
// LOGICAL TESTS — unintended properties, stronger claims, cross-function
// ========================================================================

// Test L1: Assert sequence length equals set cardinality (not in spec)
// SHOULD FAIL
proof fn test_logical_length_preserved() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().len() == s.len());
}

// Test L2: Assert no duplicate elements in sequence (not guaranteed)
// SHOULD FAIL
proof fn test_logical_no_duplicates() {
    let s = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    assert(forall |i: int, j: int|
        0 <= i < seq.len() && 0 <= j < seq.len() && i != j
        ==> seq[i] != seq[j]);
}

// Test L3: Assert specific ordering in sequence (not guaranteed)
// SHOULD FAIL
proof fn test_logical_ordering() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    assert(forall |i: int, j: int|
        0 <= i < j < seq.len() ==> seq[i] < seq[j]);
}

// Test L4: Assert biconditional holds for infinite set (over-generalization)
// SHOULD FAIL
proof fn test_logical_infinite_biconditional() {
    let s = Set::<int>::new(|x: int| x > 0);
    assert(forall |e: int| s.contains(e) <==> s.to_seq().contains(e));
}

// Test L5: Assert cross-set transfer (result for s1 used on s2)
// SHOULD FAIL
proof fn test_logical_cross_set_transfer() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s1);
    assert(s2.to_seq().contains(2int));
}

// Test L6: Assert concrete sequence structure (stronger structural claim)
// SHOULD FAIL
proof fn test_logical_concrete_sequence() {
    let s = Set::<int>::empty().insert(1int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    assert(seq =~= seq![1int]);
}

}
