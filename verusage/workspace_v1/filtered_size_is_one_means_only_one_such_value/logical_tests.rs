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

// === Logical Tests ===

// LOGICAL TEST 1: filter(f).len() == 1 does NOT imply m.len() == 1
// The multiset can have many non-matching elements; this is a strictly stronger claim
// SHOULD FAIL
proof fn test_logical_stronger_multiset_len(m: Multiset<int>, f: spec_fn(int) -> bool)
    requires m.filter(f).len() == 1,
{
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.len() == 1);
}

// LOGICAL TEST 2: filter(f).len() == 1 does NOT constrain a different predicate g
// Cross-predicate reasoning: knowing about f says nothing about g
// SHOULD FAIL
proof fn test_logical_cross_predicate(m: Multiset<int>, f: spec_fn(int) -> bool, g: spec_fn(int) -> bool)
    requires m.filter(f).len() == 1,
{
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(m.filter(g).len() <= 1);
}

// LOGICAL TEST 3: filter(f).len() == 1 does NOT imply all elements of m satisfy f
// Over-generalization: only one element satisfies f, not all of them
// SHOULD FAIL
proof fn test_logical_all_elements_satisfy_f(m: Multiset<int>, f: spec_fn(int) -> bool)
    requires m.filter(f).len() == 1, m.len() > 1,
{
    filtered_size_is_one_means_only_one_such_value(m, f);
    assert(forall |v: int| m.contains(v) ==> f(v));
}

}
