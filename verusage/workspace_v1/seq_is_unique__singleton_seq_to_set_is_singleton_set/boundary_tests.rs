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

    // SHOULD FAIL: Empty sequence should not convert to a singleton set
    // Boundary: tests whether the postcondition accidentally extends to empty sequences
    proof fn test_empty_seq_to_singleton_set()
    {
        let empty: Seq<int> = Seq::empty();
        assert(empty.to_set() == set![0int]);
    }

    // SHOULD FAIL: Two-element sequence with distinct values should not equal a singleton set
    // Boundary: tests whether multi-element sequences collapse incorrectly
    proof fn test_two_element_seq_to_singleton_set()
    {
        let s = seq![1int, 2int];
        assert(s.to_set() == set![1int]);
    }

    // SHOULD FAIL: Singleton seq should not map to a set with a different element
    // Boundary: tests whether the element identity is preserved
    proof fn test_singleton_seq_wrong_element()
    {
        singleton_seq_to_set_is_singleton_set(1int);
        assert(seq![1int].to_set() == set![2int]);
    }

    // SHOULD FAIL: Empty sequence to_set should not equal a non-empty set with arbitrary value
    // Boundary: tests empty-sequence edge case with negative value
    proof fn test_empty_seq_to_negative_singleton()
    {
        let empty: Seq<int> = Seq::empty();
        assert(empty.to_set() == set![-1int]);
    }
}
