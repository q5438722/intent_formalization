use vstd::prelude::*;
use vstd::set_lib::*;

fn main() {}

verus! {

    // Specification under test
    pub proof fn singleton_seq_to_set_is_singleton_set<T>(x: T)
        ensures seq![x].to_set() == set![x]
    {
        let seq1 = seq![x];
        let set1 = seq1.to_set();
        let set2 = set![x];
        assert forall |y| set1.contains(y) <==> set2.contains(y) by
        {
            if y == x {
                assert (seq1[0] == y);
                assert (set1.contains(y));
            }
        }
        assert_sets_equal!(seq![x].to_set(), set![x]);
    }

    // SHOULD FAIL: Negation of postcondition — seq![x].to_set() should equal set![x], not differ
    // Mutation: flips equality to inequality
    proof fn test_negation_of_postcondition()
    {
        let x: int = 42;
        singleton_seq_to_set_is_singleton_set(x);
        assert(seq![x].to_set() != set![x]);
    }

    // SHOULD FAIL: Superset mutation — singleton seq should not map to a two-element set
    // Mutation: adds an extra element to the expected output set
    proof fn test_superset_mutation()
    {
        let x: int = 1;
        singleton_seq_to_set_is_singleton_set(x);
        assert(seq![x].to_set() == set![1int, 2int]);
    }

    // SHOULD FAIL: Empty set mutation — singleton seq should not map to the empty set
    // Mutation: replaces the expected output with empty set
    proof fn test_empty_set_mutation()
    {
        let x: int = 5;
        singleton_seq_to_set_is_singleton_set(x);
        assert(seq![x].to_set() == Set::<int>::empty());
    }

    // SHOULD FAIL: Wrong element mutation — seq![x].to_set() should not equal set of a different value
    // Mutation: substitutes the expected element with a different concrete value
    proof fn test_wrong_element_mutation()
    {
        let x: int = 7;
        singleton_seq_to_set_is_singleton_set(x);
        assert(seq![x].to_set() == set![8int]);
    }
}
