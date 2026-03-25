use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Original function (copied from target) =====

#[verifier::spinoff_prover]
pub proof fn lemma_to_set_distributes_over_addition<A>(s: Seq<A>, t: Seq<A>)
    ensures (s+t).to_set() == s.to_set() + t.to_set()
{
    let left = (s+t).to_set();
    let right = s.to_set() + t.to_set();
    assert forall |x| right.contains(x) implies left.contains(x) by {
        assert(s.to_set()+t.to_set() == s.to_set().union(t.to_set()));
        if s.to_set().contains(x) {
            let si = choose |si| 0<=si<s.len() && s[si] == x;
            assert((s+t)[si] == x);
        } else {
            let ti = choose |ti| 0<=ti<t.len() && t[ti] == x;
            assert((s+t)[s.len() + ti] == x);
        }
    }
    assert_sets_equal!(left, right);
}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutate union to intersection — for disjoint sequences the intersection
// is empty, but the union is non-empty. Claiming equality with intersection should fail.
// SHOULD FAIL
proof fn test_mutation_intersection_instead_of_union() {
    let s = seq![1int, 2int];
    let t = seq![3int, 4int];
    lemma_to_set_distributes_over_addition(s, t);
    // The spec guarantees (s+t).to_set() == s.to_set() + t.to_set() (union).
    // Replacing union with intersection is incorrect for disjoint inputs.
    assert((s + t).to_set() =~= s.to_set().intersect(t.to_set()));
}

// Test 2: Negate the postcondition — directly assert the ensures clause is false.
// SHOULD FAIL
proof fn test_mutation_negate_postcondition(s: Seq<int>, t: Seq<int>) {
    lemma_to_set_distributes_over_addition(s, t);
    // The lemma guarantees equality. Asserting inequality should fail.
    assert(!((s + t).to_set() =~= (s.to_set() + t.to_set())));
}

// Test 3: Assert result set is missing an element from t.
// The union must contain all elements from t; claiming one is absent is wrong.
// SHOULD FAIL
proof fn test_mutation_missing_element_from_t() {
    let s = seq![10int];
    let t = seq![20int, 30int];
    lemma_to_set_distributes_over_addition(s, t);
    // 30 is in t, hence in t.to_set(), hence in the union.
    // Asserting it's not in the result should fail.
    assert(!(s + t).to_set().contains(30int));
}

}
