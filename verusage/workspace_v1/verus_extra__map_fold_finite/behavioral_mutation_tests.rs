use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Definitions copied from target =====

pub open spec fn set_fold<A, B>(s: Set<A>, zero: B, f: spec_fn(B, A) -> B) -> B
    recommends s.finite()
    decreases s.len()
{
    if s.finite() {
        if s.len() == 0 {
            zero
        } else {
            let a = s.choose();
            f(set_fold(s.remove(a), zero, f), a)
        }
    } else {
        zero
    }
}

spec fn map_fold<A, B>(s: Set<A>, f: spec_fn(A) -> B) -> Set<B>
    recommends s.finite()
{
    set_fold(s, Set::empty(), |s1: Set<B>, a: A| s1.insert(f(a)))
}

#[verifier::spinoff_prover]
proof fn map_fold_finite<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    requires s.finite()
    ensures map_fold(s, f).finite()
    decreases s.len()
{
    if s.len() == 0 {
        return;
    } else {
        let a = s.choose();
        map_fold_finite(s.remove(a), f);
        return;
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Negate the postcondition — claim result is NOT finite
// SHOULD FAIL
proof fn test_negated_postcondition(s: Set<int>, f: spec_fn(int) -> int)
    requires s.finite()
    ensures !map_fold(s, f).finite()
{
    map_fold_finite(s, f);
}

// Test 2: Mutate cardinality — claim len equals s.len() + 1 (off by one)
// SHOULD FAIL
proof fn test_wrong_cardinality_mutation(s: Set<int>, f: spec_fn(int) -> int)
    requires s.finite(), s.len() > 0
    ensures map_fold(s, f).len() == s.len() + 1
{
    map_fold_finite(s, f);
}

// Test 3: Claim non-empty set maps to empty set (wrong value)
// SHOULD FAIL
proof fn test_nonempty_maps_to_empty(s: Set<int>, f: spec_fn(int) -> int)
    requires s.finite(), s.len() > 0
    ensures map_fold(s, f) =~= Set::<int>::empty()
{
    map_fold_finite(s, f);
}

}
