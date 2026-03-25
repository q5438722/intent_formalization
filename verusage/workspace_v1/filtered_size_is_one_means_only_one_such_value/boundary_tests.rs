use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// === Spec under test (external_body to test ensures only) ===

#[verifier::external_body]
pub proof fn len_is_zero_means_count_for_each_value_is_zero<V>(m: Multiset<V>)
    ensures (forall |v| m.count(v) == 0) == (m.len() == 0),
{unimplemented!()}

#[verifier::external_body]
pub proof fn filtered_size_is_zero_means_no_such_value<V>(m: Multiset<V>, f: spec_fn(V) -> bool)
    ensures (m.filter(f).len() == 0) == (forall |v: V| !(#[trigger] m.contains(v) && f(v)))
{ unimplemented!()}

#[verifier::external_body]
pub proof fn filtered_size_is_one_means_only_one_such_value<V>(m: Multiset<V>, f: spec_fn(V) -> bool)
    ensures
        (m.filter(f).len() == 1) == {
            &&& exists |v| #[trigger] m.contains(v) && f(v)
            &&& forall |v| #[trigger] m.contains(v) && f(v) ==> {
                &&& m.count(v) == 1
                &&& forall |u: V| #[trigger] m.contains(u) && f(u) ==> u == v
            }
        }
{ unimplemented!() }

// === Boundary Tests ===

// BOUNDARY TEST 1: Empty multiset cannot have filter length 1
// SHOULD FAIL
proof fn test_boundary_empty_multiset_filter_len_one() {
    let m: Multiset<int> = Multiset::empty();
    let f = |v: int| true;
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.filter(f).len() == 1);
}

// BOUNDARY TEST 2: Multiset with duplicate element (count == 2) cannot have filter length 1
// SHOULD FAIL
proof fn test_boundary_duplicate_element_filter_len_one() {
    let m: Multiset<int> = Multiset::empty().insert(5).insert(5);
    let f = |v: int| v == 5;
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.filter(f).len() == 1);
}

// BOUNDARY TEST 3: Non-empty multiset with no matching elements cannot have filter length 1
// SHOULD FAIL
proof fn test_boundary_no_match_filter_len_one() {
    let m: Multiset<int> = Multiset::empty().insert(3);
    let f = |v: int| v == 5;
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.filter(f).len() == 1);
}

}
