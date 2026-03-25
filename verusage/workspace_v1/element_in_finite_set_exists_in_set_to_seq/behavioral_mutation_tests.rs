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

// SHOULD FAIL: claim the resulting sequence is empty (contradicts element membership)
proof fn test_mutation_seq_is_empty() {
    let s: Set<int> = Set::empty().insert(5);
    let e: int = 5;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    assert(s.to_seq().len() == 0);
}

// SHOULD FAIL: claim the element equals a different concrete value after conversion
proof fn test_mutation_wrong_element_value() {
    let s: Set<int> = Set::empty().insert(10);
    let e: int = 10;
    element_in_finite_set_exists_in_set_to_seq(s, e);
    // Singleton set: to_seq() should be [10], claim it contains 20 instead
    assert(s.to_seq()[0] == 20);
}

}
