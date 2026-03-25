use vstd::prelude::*;

fn main() {}

verus! {

// ===== Original function (copied from target) =====

pub proof fn a_submap_of_a_finite_map_is_finite<K, V>(m1: Map<K, V>, m2: Map<K, V>)
    requires
        m1.submap_of(m2),
        m2.dom().finite(),
    ensures
        m1.dom().finite(),
{
    assert(m1.dom() === m2.dom().intersect(m1.dom()));
}

// ========== BOUNDARY TESTS ==========

// Test 1: m1 is NOT a submap of m2 — violates first precondition.
// m1 contains key 1 which is not in m2, so submap_of fails.
// SHOULD FAIL
proof fn test_boundary_not_submap() {
    let m1 = Map::<int, int>::empty().insert(1int, 10int);
    let m2 = Map::<int, int>::empty().insert(2int, 20int);
    a_submap_of_a_finite_map_is_finite(m1, m2);
}

// Test 2: m2's domain is NOT finite — violates second precondition.
// Even though m1 is a submap, without m2.dom().finite() the call is invalid.
// SHOULD FAIL
proof fn test_boundary_m2_not_finite(m1: Map<int, int>, m2: Map<int, int>)
    requires
        m1.submap_of(m2),
        !m2.dom().finite(),
{
    a_submap_of_a_finite_map_is_finite(m1, m2);
}

// Test 3: Both preconditions violated — m1 not a submap AND m2 not finite.
// Neither requirement is satisfied.
// SHOULD FAIL
proof fn test_boundary_both_violated(m1: Map<int, int>, m2: Map<int, int>)
    requires
        !m1.submap_of(m2),
        !m2.dom().finite(),
{
    a_submap_of_a_finite_map_is_finite(m1, m2);
}

}
