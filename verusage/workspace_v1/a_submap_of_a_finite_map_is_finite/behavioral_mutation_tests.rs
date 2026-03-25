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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Negate the postcondition — assert m1's domain is NOT finite.
// The ensures says m1.dom().finite(), so negating it should be rejected.
// SHOULD FAIL
proof fn test_mutation_negate_finite(m1: Map<int, int>, m2: Map<int, int>)
    requires
        m1.submap_of(m2),
        m2.dom().finite(),
{
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(!m1.dom().finite());
}

// Test 2: Mutate to claim domain equality — m1's domain equals m2's domain.
// The spec only guarantees finiteness of m1.dom(), NOT that it equals m2.dom().
// For a proper submap, this is false.
// SHOULD FAIL
proof fn test_mutation_domain_equality() {
    let m1 = Map::<int, int>::empty().insert(1int, 10int);
    let m2 = Map::<int, int>::empty().insert(1int, 10int).insert(2int, 20int);
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(m1.dom() =~= m2.dom());
}

// Test 3: Mutate to claim map equality — m1 equals m2.
// The spec guarantees nothing about the equality of the two maps.
// A proper submap is strictly smaller than the supermap.
// SHOULD FAIL
proof fn test_mutation_map_equality() {
    let m1 = Map::<int, int>::empty().insert(1int, 10int);
    let m2 = Map::<int, int>::empty().insert(1int, 10int).insert(2int, 20int);
    a_submap_of_a_finite_map_is_finite(m1, m2);
    assert(m1 =~= m2);
}

}
