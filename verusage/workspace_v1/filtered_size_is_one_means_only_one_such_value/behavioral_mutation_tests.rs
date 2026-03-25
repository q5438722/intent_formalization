use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

// === Spec under test (external_body to test ensures only) ===

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

// === Behavioral Mutation Tests ===

// MUTATION TEST 1: When filter len == 1, the matching value must have count == 1, NOT >= 2
// Mutates the count requirement from == 1 to >= 2
// SHOULD FAIL
proof fn test_mutation_wrong_count(m: Multiset<int>, f: spec_fn(int) -> bool)
    requires m.filter(f).len() == 1,
{
    filtered_size_is_one_means_only_one_such_value(m, f);
    let v = m.filter(f).choose();
    assert(m.count(v) >= 2);
}

// MUTATION TEST 2: When filter len == 1, there cannot be two distinct matching values
// Mutates uniqueness: claims a second distinct match exists
// SHOULD FAIL
proof fn test_mutation_non_unique_match(m: Multiset<int>, f: spec_fn(int) -> bool)
    requires m.filter(f).len() == 1,
{
    filtered_size_is_one_means_only_one_such_value(m, f);
    let v = m.filter(f).choose();
    assert(exists |u: int| u != v && #[trigger] m.contains(u) && f(u));
}

// MUTATION TEST 3: A valid singleton filter result cannot have length != 1
// Flips the equivalence: asserts filter len != 1 for a clearly valid input
// SHOULD FAIL
proof fn test_mutation_flip_result() {
    let m: Multiset<int> = Multiset::empty().insert(5);
    let f = |v: int| v == 5;
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.filter(f).len() != 1);
}

}
