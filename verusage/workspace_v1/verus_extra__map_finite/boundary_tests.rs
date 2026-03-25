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

// === BOUNDARY TESTS ===

// BOUNDARY TEST 1: Infinite set — Set::new(|i| true) is not finite.
// Calling map_finite violates requires s.finite().
// SHOULD FAIL
proof fn boundary_test_infinite_set()
{
    let s = Set::<int>::new(|i: int| true);
    let f = |x: int| -> int { x + 1 };
    map_finite(s, f);
}

// BOUNDARY TEST 2: Predicate-defined set — not provably finite in Verus
// even though it describes a bounded range.
// SHOULD FAIL
proof fn boundary_test_predicate_set_not_provably_finite()
{
    let s = Set::<int>::new(|i: int| 0 <= i && i < 10);
    let f = |x: int| -> int { x * 2 };
    map_finite(s, f);
}

// BOUNDARY TEST 3: Call map_fold_finite with an infinite set (non-negative ints).
// Violates requires s.finite().
// SHOULD FAIL
proof fn boundary_test_map_fold_finite_infinite()
{
    let s = Set::<int>::new(|i: int| i >= 0);
    let f = |x: int| -> int { x };
    map_fold_finite(s, f);
}

}
