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

#[verifier::external_body]
#[verifier::spinoff_prover]
proof fn map_fold_ok<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    requires s.finite()
    ensures map_fold(s, f) =~= s.map(f)
    decreases s.len()
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier::spinoff_prover]
proof fn map_fold_finite<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    requires s.finite()
    ensures map_fold(s, f).finite()
    decreases s.len()
{
    unimplemented!()
}

#[verifier::spinoff_prover]
pub proof fn map_finite<A, B>(s: Set<A>, f: spec_fn(A) -> B)
requires
    s.finite(),
ensures
    s.map(f).finite(),
{
    map_fold_ok(s, f);
    map_fold_finite(s, f);
}

// === LOGICAL TESTS ===

// LOGICAL TEST 1: Cardinality preservation is NOT guaranteed for non-injective f.
// Constant function f(x) = 0 maps {1,2,3} to {0}, so len = 1, not 3.
// map_finite only ensures finiteness, not cardinality preservation.
// SHOULD FAIL
proof fn logical_cardinality_not_preserved()
{
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    let f = |x: int| -> int { 0int };
    map_finite(s, f);
    assert(s.map(f).len() == s.len());
}

// LOGICAL TEST 2: Cross-function equivalence — different functions yield different results.
// map({1,2}, x+1) = {2,3} ≠ map({1,2}, x*10) = {10,20}.
// The spec should not allow equating results of different mappings.
// SHOULD FAIL
proof fn logical_cross_function_equivalence()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x + 1 };
    let g = |x: int| -> int { x * 10 };
    map_finite(s, f);
    map_finite(s, g);
    assert(s.map(f) =~= s.map(g));
}

// LOGICAL TEST 3: Finiteness of one mapped set does NOT transfer to unrelated sets.
// Proving s1.map(f).finite() should not make s2.map(g).finite() provable
// when s2 is infinite.
// SHOULD FAIL
proof fn logical_finiteness_not_transferable()
{
    let s1 = Set::<int>::empty().insert(1int);
    let f = |x: int| -> int { x + 1 };
    map_finite(s1, f);
    // s2 is infinite — finiteness of s1.map(f) should not help
    let s2 = Set::<int>::new(|i: int| true);
    let g = |x: int| -> int { x };
    assert(s2.map(g).finite());
}

}
