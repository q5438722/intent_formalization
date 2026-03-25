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

// === BOUNDARY TESTS ===

// BOUNDARY TEST 1: Violate precondition — infinite set (all integers)
// The requires clause demands s.finite(), which is false for Set::new(|i| true).
// SHOULD FAIL
proof fn boundary_test_infinite_set()
{
    let s = Set::<int>::new(|i: int| true);
    let f = |x: int| -> int { x + 1 };
    map_fold_ok(s, f);
}

// BOUNDARY TEST 2: Violate precondition — predicate-defined set (not provably finite)
// Set::new with a bounded predicate is still not provably finite in Verus
// without explicit construction via insert.
// SHOULD FAIL
proof fn boundary_test_predicate_set_not_provably_finite()
{
    let s = Set::<int>::new(|i: int| 0 <= i && i < 10);
    let f = |x: int| -> int { x * 2 };
    map_fold_ok(s, f);
}

// BOUNDARY TEST 3: Assert wrong membership for empty set
// map_fold on empty set should yield empty set; claiming it contains 0 is wrong.
// SHOULD FAIL
proof fn boundary_test_empty_set_contains_element()
{
    let s = Set::<int>::empty();
    let f = |x: int| -> int { x + 1 };
    map_fold_ok(s, f);
    assert(map_fold(s, f).contains(0int));
}

}
