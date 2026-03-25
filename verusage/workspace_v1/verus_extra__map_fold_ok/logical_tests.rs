use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// === Original definitions (included for self-contained compilation) ===

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
proof fn map_fold_ok<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    requires s.finite()
    ensures map_fold(s, f) =~= s.map(f)
    decreases s.len()
{
    if s.len() == 0 {
        return;
    } else {
        let a = s.choose();
        map_fold_ok(s.remove(a), f);
        return;
    }
}

// === LOGICAL TESTS ===

// LOGICAL TEST 1: Cardinality preservation is NOT guaranteed for non-injective f.
// Constant function f(x) = 0 maps {1,2,3} to {0}, so len(result) = 1 ≠ 3.
// SHOULD FAIL
proof fn logical_cardinality_preserved_noninjective()
{
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    let f = |x: int| -> int { 0int };
    map_fold_ok(s, f);
    assert(map_fold(s, f).len() == s.len());
}

// LOGICAL TEST 2: Subset preservation without proof.
// Even though s1 ⊆ s2 implies map(s1,f) ⊆ map(s2,f), this is NOT proven
// by map_fold_ok. Asserting it without a proof should fail.
// SHOULD FAIL
proof fn logical_subset_preservation_unproven()
{
    let s1 = Set::<int>::empty().insert(1int);
    let s2 = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x + 1 };
    // We do NOT call map_fold_ok — try to assert the property directly
    assert(map_fold(s1, f).subset_of(map_fold(s2, f)));
}

// LOGICAL TEST 3: Cross-function misuse — asserting equivalence of map_fold
// under two different functions.
// map_fold(s, f) and map_fold(s, g) should differ when f ≠ g.
// SHOULD FAIL
proof fn logical_cross_function_equivalence()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x + 1 };
    let g = |x: int| -> int { x * 10 };
    map_fold_ok(s, f);
    map_fold_ok(s, g);
    // map_fold(s, f) = {2,3}, map_fold(s, g) = {10,20} — NOT equal
    assert(map_fold(s, f) =~= map_fold(s, g));
}

}
