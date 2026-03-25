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

// SHOULD FAIL: both preconditions violated (infinite + element not provably in set)
proof fn test_boundary_both_preconditions_violated() {
    let s: Set<int> = Set::new(|i: int| i > 100);
    let e: int = 0;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().contains(e));
}

}
