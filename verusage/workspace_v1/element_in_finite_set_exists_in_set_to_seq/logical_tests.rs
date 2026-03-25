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
    // Claim the sequence preserves insertion order or is sorted
    assert(s.to_seq()[0] == 1);
}

// SHOULD FAIL: spec does not guarantee the postcondition without calling the lemma
proof fn test_logical_postcondition_without_lemma() {
    let s: Set<int> = Set::empty().insert(7).insert(8);
    let e: int = 7;
    // Do NOT call the lemma — just assert the conclusion directly
    assert(s.to_seq().contains(e));
}

// SHOULD FAIL: spec does not establish that to_seq() on the superset
// preserves element presence from a subset
proof fn test_logical_cross_set_transfer() {
    let s1: Set<int> = Set::empty().insert(1).insert(2);
    let s2: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s1, e);
    // Calling the lemma on s1 does NOT establish the property on s2
    assert(s2.to_seq().contains(e));
}

// SHOULD FAIL: spec does not guarantee to_seq() has no duplicates (not stated in postcondition)
proof fn test_logical_no_duplicate_in_seq() {
    let s: Set<int> = Set::empty().insert(1).insert(2).insert(3);
    let e: int = 1;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    let seq = s.to_seq();
    // Claim: e appears at most once — not guaranteed by this lemma's spec
    assert forall |i: int, j: int|
        0 <= i < seq.len() && 0 <= j < seq.len() && seq[i] == e && seq[j] == e
        implies i == j by {};
}

}
