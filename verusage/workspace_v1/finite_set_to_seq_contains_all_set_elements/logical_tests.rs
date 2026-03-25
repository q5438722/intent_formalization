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

// ====== Logical Tests ======

// Test 1: Assert sequence length equals set cardinality (not guaranteed by spec)
// The spec only establishes element containment equivalence, not length preservation
// SHOULD FAIL
proof fn test_logical_length_preserved() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    assert(s.to_seq().len() == s.len());
}

// Test 2: Assert no duplicate elements in the sequence (not guaranteed by spec)
// The spec says containment is preserved, but doesn't rule out duplicates
// SHOULD FAIL
proof fn test_logical_no_duplicates() {
    let s = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    assert(forall |i: int, j: int|
        0 <= i < seq.len() && 0 <= j < seq.len() && i != j
        ==> seq[i] != seq[j]);
}

// Test 3: Assert specific ordering of elements in sequence
// The spec says nothing about element ordering
// SHOULD FAIL
proof fn test_logical_ordering() {
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    // Assert elements appear in ascending order
    assert(forall |i: int, j: int|
        0 <= i < j < seq.len() ==> seq[i] < seq[j]);
}

// Test 4: Assert property holds for infinite sets without finiteness (over-generalization)
// SHOULD FAIL
proof fn test_logical_infinite_biconditional() {
    let s = Set::<int>::new(|x: int| x > 0); // infinite set
    // Try to assert biconditional without calling the lemma (no finite precondition)
    assert(forall |e: int| s.contains(e) <==> s.to_seq().contains(e));
}

// Test 5: Assert the lemma result transfers across different sets (cross-function misuse)
// Proving the lemma for s1 should not help prove things about s2
// SHOULD FAIL
proof fn test_logical_cross_set_transfer() {
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(1int).insert(2int);
    finite_set_to_seq_contains_all_set_elements(s1);
    // Try to use s1's result to conclude about s2 without calling lemma on s2
    assert(s2.to_seq().contains(2int));
}

// Test 6: Assert the sequence is a specific concrete value (stronger structural claim)
// SHOULD FAIL
proof fn test_logical_concrete_sequence() {
    let s = Set::<int>::empty().insert(1int);
    finite_set_to_seq_contains_all_set_elements(s);
    let seq = s.to_seq();
    // Assert seq is exactly the sequence [1] — this is a stronger claim than containment
    assert(seq =~= seq![1int]);
}

}
