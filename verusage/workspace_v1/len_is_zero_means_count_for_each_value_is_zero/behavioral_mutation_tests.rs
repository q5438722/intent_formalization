use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// ---- Original specification under test ----

#[verifier::external_body]
pub proof fn len_is_zero_means_count_for_each_value_is_zero<V>(m: Multiset<V>)
    ensures (forall |v| m.count(v) == 0) == (m.len() == 0),
{ unimplemented!() }

// ============================================================
// (2) BEHAVIORAL MUTATION TESTS — Mutate expected relations
// ============================================================

// SHOULD FAIL: Flip the biconditional — all counts zero implies len > 0
proof fn test_mutation_flip_biconditional() {
    let m: Multiset<int> = Multiset::empty();
    len_is_zero_means_count_for_each_value_is_zero(m);
    // For empty m, all counts are 0 and len == 0. Assert the flipped version.
    assert((forall |v: int| m.count(v) == 0) ==> m.len() > 0);
}

// SHOULD FAIL: Negate one direction — len > 0 should imply exists v with count > 0,
// but here we assert len > 0 implies all counts are zero
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
    // Spec only says NOT all counts are zero; does not say every count is positive
    assert(forall |v: int| m.count(v) > 0);
}

// SHOULD FAIL: Mutate count threshold — claim len == 0 iff all counts <= 1 (weaker predicate)
// For singleton(1): count(1) == 1 so all counts <= 1 is TRUE, but len == 1 != 0 so len == 0 is FALSE
proof fn test_mutation_weaker_count_threshold() {
    let m: Multiset<int> = Multiset::singleton(1);
    len_is_zero_means_count_for_each_value_is_zero(m);
    assert((forall |v: int| m.count(v) <= 1) == (m.len() == 0));
}

// SHOULD FAIL: Assert count of a specific value equals len
proof fn test_mutation_count_equals_len() {
    let m: Multiset<int> = Multiset::singleton(1).add(Multiset::singleton(2));
    len_is_zero_means_count_for_each_value_is_zero(m);
    // len is 2, count(1) is 1, so they are not equal
    assert(m.count(1) == m.len());
}

}
