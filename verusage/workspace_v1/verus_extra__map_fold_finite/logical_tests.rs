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

// ===== LOGICAL TESTS =====

// Test 1: Cardinality preservation — not guaranteed (f may not be injective)
// SHOULD FAIL
proof fn test_cardinality_preservation(s: Set<int>, f: spec_fn(int) -> int)
    requires s.finite(), s.len() > 1
    ensures map_fold(s, f).len() == s.len()
{
    map_fold_finite(s, f);
}

// Test 2: Element containment — not entailed by the spec
// (true semantically but opaque map_fold prevents derivation)
// SHOULD FAIL
proof fn test_element_containment(s: Set<int>, f: spec_fn(int) -> int, x: int)
    requires s.finite(), s.contains(x)
    ensures map_fold(s, f).contains(f(x))
{
    map_fold_finite(s, f);
}

// Test 3: Distributivity over union — not entailed by the spec
// SHOULD FAIL
proof fn test_distributivity_over_union(s1: Set<int>, s2: Set<int>, f: spec_fn(int) -> int)
    requires s1.finite(), s2.finite()
    ensures map_fold(s1.union(s2), f) =~= map_fold(s1, f).union(map_fold(s2, f))
{
    map_fold_finite(s1, f);
    map_fold_finite(s2, f);
}

}
