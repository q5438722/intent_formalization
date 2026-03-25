use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// ---- Original specification under test ----

#[verifier::external_body]
pub proof fn len_is_zero_means_count_for_each_value_is_zero<V>(m: Multiset<V>)
    ensures (forall |v| m.count(v) == 0) == (m.len() == 0),
{ unimplemented!() }

// ============================================================
// (1) BOUNDARY TESTS — Edge cases and invalid input assertions
// ============================================================

// SHOULD FAIL: Empty multiset has length 0, but assert it has positive length
proof fn test_boundary_empty_multiset_positive_len() {
    let m: Multiset<int> = Multiset::empty();
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.len() > 0);
}

// SHOULD FAIL: Singleton multiset has length 1, but assert all counts are zero
proof fn test_boundary_singleton_all_counts_zero() {
    let m: Multiset<int> = Multiset::singleton(42);
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(forall |v: int| m.count(v) == 0);
}

// SHOULD FAIL: After inserting, claim length is still zero
proof fn test_boundary_insert_then_claim_zero_len() {
    let m: Multiset<int> = Multiset::empty().insert(7);
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.len() == 0);
}

// SHOULD FAIL: Multi-element multiset — claim all counts zero
proof fn test_boundary_two_elements_all_counts_zero() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(forall |v: int| m.count(v) == 0);
}

// SHOULD FAIL: Empty multiset — claim some element has positive count
proof fn test_boundary_empty_has_positive_count() {
    let m: Multiset<int> = Multiset::empty();
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.count(0) > 0);
}

}
