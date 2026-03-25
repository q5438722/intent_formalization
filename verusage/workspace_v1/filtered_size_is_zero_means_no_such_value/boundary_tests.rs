use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// ---- Original specification under test ----

#[verifier::external_body]
pub proof fn len_is_zero_means_count_for_each_value_is_zero<V>(m: Multiset<V>)
    ensures (forall |v| m.count(v) == 0) == (m.len() == 0),
{ unimplemented!() }

pub proof fn filtered_size_is_zero_means_no_such_value<V>(m: Multiset<V>, f: spec_fn(V) -> bool)
    ensures (m.filter(f).len() == 0) == (forall |v: V| !(#[trigger] m.contains(v) && f(v)))
{
    if forall |v: V| !(#[trigger] m.contains(v) && f(v)) {
        assert forall |v| m.filter(f).count(v) == 0 by {
            if m.contains(v) {
                assert(!f(v));
            }
        }
        len_is_zero_means_count_for_each_value_is_zero(m.filter(f));
    }
    if !forall |v: V| !(#[trigger] m.contains(v) && f(v)) {
        let v = choose |v| m.contains(v) && f(v);
        assert(m.filter(f).contains(v));
    }
}

// ============================================================
// (1) BOUNDARY TESTS — Violate preconditions / edge cases
// ============================================================

// SHOULD FAIL: Empty multiset — filter len is 0, but assert it is non-zero
proof fn test_boundary_empty_multiset_nonzero_filter_len() {
    let m: Multiset<int> = Multiset::empty();
    let f = |x: int| true;
    filtered_size_is_zero_means_no_such_value(m, f);
    // The spec tells us m.filter(f).len() == 0 for empty m. Assert the opposite.
    assert(m.filter(f).len() > 0);
}

// SHOULD FAIL: Singleton multiset with element satisfying f — filter len > 0, but assert it is 0
proof fn test_boundary_singleton_satisfying_filter_is_zero() {
    let m: Multiset<int> = Multiset::singleton(5);
    let f = |x: int| x == 5;
    filtered_size_is_zero_means_no_such_value(m, f);
    // m contains 5 and f(5) is true, so filter len > 0. Assert the opposite.
    assert(m.filter(f).len() == 0);
}

// SHOULD FAIL: Singleton with element NOT satisfying f — filter len is 0, then claim m contains nothing
proof fn test_boundary_singleton_not_satisfying_still_contains() {
    let m: Multiset<int> = Multiset::singleton(5);
    let f = |x: int| x == 10;
    filtered_size_is_zero_means_no_such_value(m, f);
    // Filter is empty because f(5) is false, but m still contains 5
    assert(!m.contains(5));
}

// SHOULD FAIL: Assert empty multiset contains a value after calling the lemma
proof fn test_boundary_empty_multiset_contains_value() {
    let m: Multiset<int> = Multiset::empty();
    let f = |x: int| x > 0;
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.contains(42));
}

// SHOULD FAIL: Use always-true filter on non-empty multiset and claim filter is empty
proof fn test_boundary_always_true_filter_on_nonempty() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    let f = |x: int| true;
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.filter(f).len() == 0);
}

}
