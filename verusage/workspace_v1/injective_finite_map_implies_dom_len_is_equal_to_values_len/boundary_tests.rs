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

// ===== BOUNDARY TESTS =====

// Test 1: Non-injective map — two keys map to the same value
// Violates m.is_injective() precondition
// SHOULD FAIL
proof fn test_boundary_non_injective() {
    let m = Map::<int, int>::empty().insert(1int, 1int).insert(2int, 1int);
    // m = {1->1, 2->1}, NOT injective (both keys map to 1)
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() == m.values().len());
}

// Test 2: Infinite injective map — domain is not finite
// Violates m.dom().finite() precondition
// SHOULD FAIL
proof fn test_boundary_infinite_injective(m: Map<int, int>)
    requires
        !m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() == m.values().len());
}

// Test 3: Both preconditions violated — infinite AND non-injective
// Violates both m.dom().finite() and m.is_injective()
// SHOULD FAIL
proof fn test_boundary_both_violated(m: Map<int, int>)
    requires
        !m.dom().finite(),
        !m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() == m.values().len());
}

}
