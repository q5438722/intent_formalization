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

// === BEHAVIORAL MUTATION TESTS ===

// MUTATION TEST 1: Negate postcondition of map_finite.
// map_finite guarantees s.map(f).finite(); asserting NOT finite should fail.
// SHOULD FAIL
proof fn mutation_negate_finiteness()
{
    let s = Set::<int>::empty().insert(1int).insert(2int).insert(3int);
    let f = |x: int| -> int { x + 1 };
    map_finite(s, f);
    assert(!s.map(f).finite());
}

// MUTATION TEST 2: Negate postcondition of map_fold_ok.
// map_fold_ok guarantees map_fold(s,f) =~= s.map(f); asserting inequality should fail.
// SHOULD FAIL
proof fn mutation_map_fold_not_equal_to_map()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x + 1 };
    map_fold_ok(s, f);
    assert(!(map_fold(s, f) =~= s.map(f)));
}

// MUTATION TEST 3: Negate postcondition of map_fold_finite.
// map_fold_finite guarantees map_fold(s,f).finite(); asserting NOT finite should fail.
// SHOULD FAIL
proof fn mutation_map_fold_not_finite()
{
    let s = Set::<int>::empty().insert(1int).insert(2int);
    let f = |x: int| -> int { x * 2 };
    map_fold_finite(s, f);
    assert(!map_fold(s, f).finite());
}

}
