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

// ============================================================
// (2) BEHAVIORAL MUTATION TESTS — Mutate expected relations
// ============================================================

// SHOULD FAIL: Flip the biconditional — all counts zero implies len > 0
proof fn test_mutation_flip_biconditional() {
    let m: Multiset<int> = Multiset::empty();
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert((forall |v: int| m.count(v) == 0) ==> m.len() > 0);
}

// SHOULD FAIL: Negate one direction — assert len > 0 implies all counts are zero
proof fn test_mutation_nonempty_implies_all_zero() {
    let m: Multiset<int> = Multiset::singleton(5);
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.len() > 0 ==> (forall |v: int| m.count(v) == 0));
}

// SHOULD FAIL: Strengthen to exact count — if len > 0, assert ALL values have count > 0
proof fn test_mutation_nonempty_all_counts_positive(m: Multiset<int>)
    requires m.len() > 0,
{
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(forall |v: int| m.count(v) > 0);
}

// SHOULD FAIL: Mutate count threshold — claim len == 0 iff all counts <= 1
proof fn test_mutation_weaker_count_threshold() {
    let m: Multiset<int> = Multiset::singleton(1);
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert((forall |v: int| m.count(v) <= 1) == (m.len() == 0));
}

// SHOULD FAIL: Assert count of a specific value equals len
proof fn test_mutation_count_equals_len() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.count(1) == m.len());
}

// ============================================================
// (3) LOGICAL TESTS — Properties NOT explicitly guaranteed
// ============================================================

// SHOULD FAIL: Derive false — the axiom should not be unsound
proof fn test_logical_derive_false()
    ensures false,
{
    let m: Multiset<int> = Multiset::singleton(1);
    len_is_zero_means_count_for_each_value_is_zero(m);
}

// SHOULD FAIL: Stronger inequality — len > 0 implies len >= 2
proof fn test_logical_len_at_least_two(m: Multiset<int>)
    requires m.len() > 0,
{
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.len() >= 2);
}

// SHOULD FAIL: Two multisets with same length must be equal
proof fn test_logical_same_len_implies_equal(m1: Multiset<int>, m2: Multiset<int>)
    requires m1.len() == m2.len(),
{
    len_is_zero_means_count_for_each_value_is_zero(m1);
    len_is_zero_means_count_for_each_value_is_zero(m2);
    assert(m1 =~= m2);
}

// SHOULD FAIL: All multisets are empty — axiom should not prove universal emptiness
proof fn test_logical_all_multisets_empty()
    ensures forall |m: Multiset<int>| m.len() == 0,
{
    assert forall |m: Multiset<int>| m.len() == 0 by {
        len_is_zero_means_count_for_each_value_is_zero(m);
    }
}

// SHOULD FAIL: Uniqueness of choose — if len > 0, only one value has nonzero count
proof fn test_logical_unique_element(m: Multiset<int>)
    requires m.len() > 0,
{
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(forall |v1: int, v2: int| m.count(v1) > 0 && m.count(v2) > 0 ==> v1 == v2);
}

// SHOULD FAIL: Count is always 0 or 1 — spec does not bound count values
proof fn test_logical_count_at_most_one(m: Multiset<int>)
{
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(forall |v: int| m.count(v) <= 1);
}

// SHOULD FAIL: len == 0 is decidable in a strong sense — claiming len is always 0
proof fn test_logical_len_always_zero(m: Multiset<int>)
{
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert(m.len() == 0);
}

}
