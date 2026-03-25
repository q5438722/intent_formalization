use vstd::prelude::*;
use vstd::set_lib::*;
use vstd::map_lib::*;

fn main() {}

verus! {

// ===== Original function (copied from target) =====

pub proof fn injective_finite_map_implies_dom_len_is_equal_to_values_len<K, V>(m: Map<K, V>)
    requires
        m.dom().finite(),
        m.is_injective(),
    ensures
        m.dom().len() == m.values().len(),
    decreases
        m.dom().len(),
{
    if m.dom().len() == 0 {
        assert(forall |k: K| !m.dom().contains(k));
        assert(forall |v: V| !m.values().contains(v));
        assert(m.values() =~= Set::empty());
    } else {
        let key = m.dom().choose();
        let value = m[key];
        let singleton_value = Set::empty().insert(value);
        let submap = m.remove(key);

        injective_finite_map_implies_dom_len_is_equal_to_values_len(submap);
        assert forall |v: V| #[trigger] submap.contains_value(v) && m.is_injective() implies v != value by {
            let k = choose |i: K| #[trigger] submap.dom().contains(i) && submap[i] == v;
            assert(k != key);
        }

        lemma_values_finite(submap);
        assert(m.values() =~= submap.values().union(singleton_value));
        lemma_set_disjoint_lens(submap.values(), singleton_value);
        assert(m.values().len() == submap.values().len() + 1);
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Mutate equality to strict greater-than
// The postcondition says dom.len == values.len, NOT dom.len > values.len
// SHOULD FAIL
proof fn test_mutation_dom_strictly_greater(m: Map<int, int>)
    requires
        m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() > m.values().len());
}

// Test 2: Mutate equality to inequality (negation)
// The postcondition says dom.len == values.len, NOT dom.len != values.len
// SHOULD FAIL
proof fn test_mutation_not_equal(m: Map<int, int>)
    requires
        m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() != m.values().len());
}

// Test 3: Mutate equality to off-by-one
// The postcondition says dom.len == values.len, NOT dom.len == values.len + 1
// SHOULD FAIL
proof fn test_mutation_off_by_one(m: Map<int, int>)
    requires
        m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() == m.values().len() + 1);
}

}
