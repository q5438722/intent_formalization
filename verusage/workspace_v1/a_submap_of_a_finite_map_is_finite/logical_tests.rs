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

// ========== LOGICAL TESTS ==========

// Test 1: Soundness — try to derive `false` from the spec.
// If the specification is consistent, calling it with valid inputs should
// not allow proving `false`.
// SHOULD FAIL
proof fn test_logical_derive_false(m1: Map<int, int>, m2: Map<int, int>)
    requires
        m1.submap_of(m2),
        m2.dom().finite(),
{
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(false);
}

// Test 2: Stronger inequality — assert submap's domain is LARGER than supermap's.
// For a submap, m1.dom() ⊆ m2.dom(), so m1.dom().len() <= m2.dom().len().
// Asserting strictly greater is an impossible claim.
// SHOULD FAIL
proof fn test_logical_submap_larger_than_supermap() {
    let m1 = Map::<int, int>::empty().insert(1int, 10int);
    let m2 = Map::<int, int>::empty().insert(1int, 10int).insert(2int, 20int);
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(m1.dom().len() > m2.dom().len());
}

// Test 3: Reverse submap — the spec does NOT guarantee m2 is a submap of m1.
// The relationship is one-directional: m1.submap_of(m2).
// Asserting m2.submap_of(m1) when m2 has extra keys should fail.
// SHOULD FAIL
proof fn test_logical_reverse_submap() {
    let m1 = Map::<int, int>::empty().insert(1int, 10int);
    let m2 = Map::<int, int>::empty().insert(1int, 10int).insert(2int, 20int);
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(m2.submap_of(m1));
}

}
