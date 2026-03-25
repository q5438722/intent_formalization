use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// ========== Original specification under test ==========

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
    assert(m.filter(f).len() > 0);
}

// SHOULD FAIL: Singleton multiset with element satisfying f — filter len > 0, but assert it is 0
proof fn test_boundary_singleton_satisfying_filter_is_zero() {
    let m: Multiset<int> = Multiset::singleton(5);
    let f = |x: int| x == 5;
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.filter(f).len() == 0);
}

// SHOULD FAIL: Singleton with element NOT satisfying f — filter is empty, then deny m contains the element
proof fn test_boundary_singleton_not_satisfying_still_contains() {
    let m: Multiset<int> = Multiset::singleton(5);
    let f = |x: int| x == 10;
    filtered_size_is_zero_means_no_such_value(m, f);
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

// ============================================================
// (2) BEHAVIORAL MUTATION TESTS — Mutate expected relations
// ============================================================

// SHOULD FAIL: Flip biconditional — claim filter len == 0 implies EXISTS v with m.contains(v) && f(v)
proof fn test_mutation_flip_biconditional() {
    let m: Multiset<int> = Multiset::singleton(3);
    let f = |x: int| x > 100;
    filtered_size_is_zero_means_no_such_value(m, f);
    let v = choose |v: int| m.contains(v) && f(v);
    assert(m.contains(v) && f(v));
}

// SHOULD FAIL: Negate the postcondition — filter len != 0 when no value satisfies f
proof fn test_mutation_negate_postcondition() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    let f = |x: int| x > 10;
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.filter(f).len() != 0);
}

// SHOULD FAIL: Strengthen to equality — claim filter length equals multiset length
proof fn test_mutation_filter_len_equals_multiset_len(m: Multiset<int>, f: spec_fn(int) -> bool)
    requires m.len() > 0,
{
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.filter(f).len() == m.len());
}

// SHOULD FAIL: Swap contains and f — claim m.contains(v) iff f(v) for all v
proof fn test_mutation_swap_contains_and_f() {
    let m: Multiset<int> = Multiset::singleton(5);
    let f = |x: int| x > 0;
    filtered_size_is_zero_means_no_such_value(m, f);
    // m.contains(1) is false but f(1) is true, so this should fail
    assert(forall |v: int| m.contains(v) == f(v));
}

// SHOULD FAIL: Weaken filter to claim filter len > multiset len
proof fn test_mutation_filter_exceeds_multiset() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    let f = |x: int| true;
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(m.filter(f).len() > m.len());
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

// SHOULD FAIL: Filter preserves all counts — not entailed by spec
proof fn test_logical_double_filter_equals_single(m: Multiset<int>, f: spec_fn(int) -> bool) {
    filtered_size_is_zero_means_no_such_value(m, f);
    assert(forall |v: int| m.filter(f).count(v) == m.count(v));
}

// SHOULD FAIL: Two multisets with same filter result must be equal (injectivity)
proof fn test_logical_filter_injective(m1: Multiset<int>, m2: Multiset<int>, f: spec_fn(int) -> bool)
    requires
        m1.filter(f).len() == m2.filter(f).len(),
{
    filtered_size_is_zero_means_no_such_value(m1, f);
    filtered_size_is_zero_means_no_such_value(m2, f);
    assert(m1 =~= m2);
}

}
