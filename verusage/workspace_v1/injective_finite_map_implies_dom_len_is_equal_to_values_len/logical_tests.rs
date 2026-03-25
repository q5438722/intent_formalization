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

// ===== LOGICAL TESTS =====

// Test 1: Over-generalization — the property does NOT hold without injectivity
// For a finite but non-injective map, dom.len != values.len in general
// (e.g. {1->1, 2->1} has dom.len=2, values.len=1)
// The spec requires injectivity, so omitting it should not allow the conclusion
// SHOULD FAIL
proof fn test_logical_holds_without_injectivity(m: Map<int, int>)
    requires
        m.dom().finite(),
        // NOTE: m.is_injective() is deliberately omitted
{
    assert(m.dom().len() == m.values().len());
}

// Test 2: Equal cardinality does NOT imply equal sets
// After the lemma, dom.len == values.len, but dom and values can be disjoint sets
// For Map<int,int> with {1->10, 2->20}: dom={1,2}, values={10,20}, same size but different
// SHOULD FAIL
proof fn test_logical_equal_len_not_equal_sets(m: Map<int, int>)
    requires
        m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    // Equal cardinality does NOT mean equal sets
    assert(m.dom() =~= m.values());
}

// Test 3: The lemma does NOT guarantee non-emptiness
// An empty map is finite and vacuously injective, with dom.len == values.len == 0
// So the spec does not rule out empty maps; asserting dom.len > 0 is not entailed
// SHOULD FAIL
proof fn test_logical_nonemptiness_not_guaranteed(m: Map<int, int>)
    requires
        m.dom().finite(),
        m.is_injective(),
{
    injective_finite_map_implies_dom_len_is_equal_to_values_len(m);
    assert(m.dom().len() > 0);
}

}
