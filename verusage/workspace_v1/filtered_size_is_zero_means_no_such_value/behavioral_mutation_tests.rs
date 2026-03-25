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
// (2) BEHAVIORAL MUTATION TESTS — Mutate expected relations
// ============================================================

// SHOULD FAIL: Flip biconditional — claim filter len == 0 implies EXISTS v with m.contains(v) && f(v)
proof fn test_mutation_flip_biconditional() {
    let m: Multiset<int> = Multiset::singleton(3);
    let f = |x: int| x > 100;
    filtered_size_is_zero_means_no_such_value(m, f);
    // Filter is empty (no value > 100), but assert there exists such a value
    let v = choose |v: int| m.contains(v) && f(v);
    assert(m.contains(v) && f(v));
}

// SHOULD FAIL: Negate the postcondition — filter len != 0 when no value satisfies f
proof fn test_mutation_negate_postcondition() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    let f = |x: int| x > 10;
    filtered_size_is_zero_means_no_such_value(m, f);
    // No value in m is > 10, so filter should be empty. Assert the opposite.
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

}
