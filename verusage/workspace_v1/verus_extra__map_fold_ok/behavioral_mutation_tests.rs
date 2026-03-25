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

// === BEHAVIORAL MUTATION TESTS ===

// MUTATION TEST 1: Assert map_fold equals the original set (not the mapped set)
// For f = x+1, map_fold({1,2,3}, f) should be {2,3,4}, NOT {1,2,3}.
// SHOULD FAIL
proof fn mutation_wrong_output_equals_original()
{
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    let f = |x: int| -> int { x + 1 };
    map_fold_ok(s, f);
    assert(map_fold(s, f) =~= s);
}

// MUTATION TEST 2: Assert a correct output element is absent
// For f = x*2, map_fold({1,2}, f) should be {2,4}. Asserting 2 is NOT present is wrong.
// SHOULD FAIL
proof fn mutation_deny_correct_element()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x * 2 };
    map_fold_ok(s, f);
    assert(!map_fold(s, f).contains(2int));
}

// MUTATION TEST 3: Assert a spurious element IS present
// For f = x+10, map_fold({1,2}, f) should be {11,12}. Asserting 1 is present is wrong.
// SHOULD FAIL
proof fn mutation_claim_spurious_element()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x + 10 };
    map_fold_ok(s, f);
    assert(map_fold(s, f).contains(1int));
}

}
