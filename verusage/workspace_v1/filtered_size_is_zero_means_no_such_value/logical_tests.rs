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
// (3) LOGICAL TESTS — Properties NOT explicitly guaranteed
// ============================================================

// SHOULD FAIL: Derive false from the axiom — axiom should not be unsound
proof fn test_logical_derive_false()
    ensures false,
{
    let m: Multiset<int> = Multiset::singleton(1);
    let f = |x: int| x > 0;
    filtered_size_is_zero_means_no_such_value(m, f);
    len_is_zero_means_count_for_each_value_is_zero(m);
}

// SHOULD FAIL: Filter is identity — spec does not imply filter(f) equals m
proof fn test_logical_filter_is_identity()
    ensures forall |m: Multiset<int>, f: spec_fn(int) -> bool|
        m.filter(f) =~= m,
{
}

// SHOULD FAIL: Filter with different predicates gives same result
proof fn test_logical_filter_predicate_irrelevance()
    ensures forall |m: Multiset<int>, f: spec_fn(int) -> bool, g: spec_fn(int) -> bool|
        m.filter(f).len() == m.filter(g).len(),
{
}

// SHOULD FAIL: Stronger bound — filter length is always exactly 0 or 1
proof fn test_logical_filter_len_at_most_one()
    ensures forall |m: Multiset<int>, f: spec_fn(int) -> bool|
        m.filter(f).len() <= 1,
{
}

// SHOULD FAIL: Cross-function misuse — use axiom to derive multiset always has len 0
proof fn test_logical_all_multisets_empty()
    ensures forall |m: Multiset<int>| m.len() == 0,
{
    assert forall |m: Multiset<int>| m.len() == 0 by {
        len_is_zero_means_count_for_each_value_is_zero(m);
    }
}

// SHOULD FAIL: Commutativity of filter composition — not entailed by spec
proof fn test_logical_double_filter_equals_single(m: Multiset<int>, f: spec_fn(int) -> bool) {
    filtered_size_is_zero_means_no_such_value(m, f);
    // Try to assert the filtered multiset has the same count as the original for ALL values
    assert(forall |v: int| m.filter(f).count(v) == m.count(v));
}

// SHOULD FAIL: Determinism-like — two multisets with same filter result must be equal
proof fn test_logical_filter_injective(m1: Multiset<int>, m2: Multiset<int>, f: spec_fn(int) -> bool)
    requires
        m1.filter(f).len() == m2.filter(f).len(),
{
    filtered_size_is_zero_means_no_such_value(m1, f);
    filtered_size_is_zero_means_no_such_value(m2, f);
    assert(m1 =~= m2);
}

}
