#![allow(unused_imports)]
use vstd::prelude::*;
use vstd::set::*;
use vstd::set_lib::*;

fn main() {}

verus! {

proof fn element_in_finite_set_exists_in_set_to_seq<A>(s: Set<A>, e: A)
    requires s.finite(), s.contains(e),
    ensures s.to_seq().contains(e),
    decreases s.len()
{
    if s.len() != 0 {
        let x = s.choose();
        if x == e {
            assert(s.to_seq() == Seq::empty().push(e) + s.remove(e).to_seq());
            assert(s.to_seq()[0] == e);
        } else {
            element_in_finite_set_exists_in_set_to_seq(s.remove(x), e);
            assert(s.to_seq().subrange(1, s.to_seq().len() as int) == s.remove(x).to_seq());
        }
    }
}

// ======================================================================
// BOUNDARY TESTS — violate preconditions
// ======================================================================

// SHOULD FAIL: infinite set violates s.finite() precondition
proof fn test_boundary_infinite_set() {
    let s: Set<int> = Set::new(|i: int| true);
    let e: int = 42;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(e));
}

// SHOULD FAIL: element not in set violates s.contains(e) precondition
proof fn test_boundary_element_not_in_set() {
    let s: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 99;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(e));
}

// SHOULD FAIL: empty set has no elements, violates s.contains(e)
proof fn test_boundary_empty_set() {
    let s: Set<int> = Set::empty();
    let e: int = 0;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(e));
}

// SHOULD FAIL: both preconditions violated (infinite + element not in set)
proof fn test_boundary_both_preconditions_violated() {
    let s: Set<int> = Set::new(|i: int| i > 100);
    let e: int = 0;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(e));
}

// ======================================================================
// BEHAVIORAL MUTATION TESTS — mutate expected outputs/relations
// ======================================================================

// SHOULD FAIL: negate the postcondition — claim e is NOT in the sequence
proof fn test_mutation_negate_postcondition() {
    let s: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 2;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(!s.to_seq().contains(e));
}

// SHOULD FAIL: claim an element NOT in the set appears in the sequence
proof fn test_mutation_absent_element_in_seq() {
    let s: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(99));
}

// SHOULD FAIL: claim the resulting sequence is empty
proof fn test_mutation_seq_is_empty() {
    let s: Set<int> = Set::empty().insert(5);
    let e: int = 5;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().len() == 0);
}

// SHOULD FAIL: claim the element at index 0 is a wrong value
proof fn test_mutation_wrong_element_value() {
    let s: Set<int> = Set::empty().insert(10);
    let e: int = 10;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq()[0] == 20);
}

// ======================================================================
// LOGICAL TESTS — unintended properties not guaranteed by spec
// ======================================================================

// SHOULD FAIL: spec does not guarantee sequence is strictly longer than set
proof fn test_logical_seq_longer_than_set() {
    let s: Set<int> = Set::empty().insert(1).insert(2);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().len() > s.len());
}

// SHOULD FAIL: spec does not guarantee any ordering in to_seq()
proof fn test_logical_ordering_guarantee() {
    let s: Set<int> = Set::empty().insert(3).insert(1).insert(2);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq()[0] == 1);
}

// SHOULD FAIL: the postcondition is not available without calling the lemma
proof fn test_logical_postcondition_without_lemma() {
    let s: Set<int> = Set::empty().insert(7).insert(8);
    let e: int = 7;
    assert(s.to_seq().contains(e));
}

// SHOULD FAIL: calling lemma on s1 does not establish properties on s2
proof fn test_logical_cross_set_transfer() {
    let s1: Set<int> = Set::empty().insert(1).insert(2);
    let s2: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s1, e);
    assert(s2.to_seq().contains(e));
}

// SHOULD FAIL: spec does not guarantee uniqueness of elements in sequence
proof fn test_logical_no_duplicate_in_seq() {
    let s: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    let seq = s.to_seq();
    assert forall |i: int, j: int|
        0 <= i < seq.len() && 0 <= j < seq.len() && seq[i] == e && seq[j] == e
        implies i == j by {};
}

}
