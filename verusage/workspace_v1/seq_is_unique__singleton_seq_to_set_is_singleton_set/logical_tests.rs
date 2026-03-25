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

    // SHOULD FAIL: The lemma should not imply that all elements are equal
    // Logical: checks that singleton set facts for different values don't collapse to equality
    proof fn test_universal_equality()
    {
        let x: int = 1;
        let y: int = 2;
        singleton_seq_to_set_is_singleton_set(x);
        singleton_seq_to_set_is_singleton_set(y);
        assert(x == y);
    }

    // SHOULD FAIL: Multi-element sequence should not collapse to singleton set for distinct elements
    // Logical: checks that the singleton lemma doesn't extend to multi-element sequences
    proof fn test_multi_element_collapse()
    {
        let x: int = 1;
        let y: int = 2;
        singleton_seq_to_set_is_singleton_set(x);
        assert(seq![x, y].to_set() == set![x]);
    }

    // SHOULD FAIL: Singleton set should not contain elements other than x
    // Logical: checks that set![x].contains(y) is not derivable for distinct x and y
    proof fn test_singleton_set_contains_other()
    {
        let x: int = 10;
        let y: int = 20;
        singleton_seq_to_set_is_singleton_set(x);
        assert(set![x].contains(y));
    }

    // SHOULD FAIL: The lemma should not imply set cardinality claims beyond what is proved
    // Logical: checks that no unintended cardinality reasoning is enabled
    proof fn test_unintended_cardinality()
    {
        let x: int = 3;
        singleton_seq_to_set_is_singleton_set(x);
        // Try to assert the set has cardinality 0 (wrong)
        assert(seq![x].to_set().len() == 0);
    }
}
