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

// ===== BOUNDARY TESTS =====

// Test 1: Call map_fold_finite without establishing s.finite()
// SHOULD FAIL
proof fn test_missing_finite_precondition(s: Set<int>, f: spec_fn(int) -> int)
    ensures map_fold(s, f).finite()
{
    map_fold_finite(s, f); // precondition s.finite() not satisfied
}

// Test 2: Prove finiteness for s1 but claim it for a different set s2
// SHOULD FAIL
proof fn test_postcondition_wrong_set(s1: Set<int>, s2: Set<int>, f: spec_fn(int) -> int)
    requires s1.finite()
    ensures map_fold(s2, f).finite()
{
    map_fold_finite(s1, f); // only proves for s1, not s2
}

// Test 3: Claim stronger postcondition than guaranteed (len == 0 for non-empty input)
// SHOULD FAIL
proof fn test_stronger_than_postcondition(s: Set<int>, f: spec_fn(int) -> int)
    requires s.finite(), s.len() > 0
    ensures map_fold(s, f).len() == 0
{
    map_fold_finite(s, f); // only guarantees finite(), not len() == 0
}

}
