use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Target lemma (copied from source) =====
#[verifier::spinoff_prover]
pub proof fn lemma_set_map_insert<A, B>(s: Set<A>, f: spec_fn(A) -> B, x: A)
    ensures s.insert(x).map(f) == s.map(f).insert(f(x))
{
    assert_sets_equal!(s.insert(x).map(f) == s.map(f).insert(f(x)), y => {
        if y == f(x) {
            assert(s.insert(x).contains(x));
        } else {
            if s.insert(x).map(f).contains(y) {
                let x0 = choose |x0| s.contains(x0) && y == f(x0);
                assert(s.map(f).contains(y));
            } else {
                if s.map(f).insert(f(x)).contains(y) {
                    let x0 = choose |x0| s.contains(x0) && y == f(x0);
                    assert(s.map(f).contains(y));
                    assert(s.insert(x).contains(x0));
                }
            }
        }
    });
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Omit the insert(f(x)) on the RHS — mutated postcondition
// True postcondition: s.insert(x).map(f) == s.map(f).insert(f(x))
// Mutated:           s.insert(x).map(f) == s.map(f)     [missing insert]
// SHOULD FAIL
proof fn test_mutation_omit_insert_rhs()
{
    let s = Set::<int>::empty();
    let f = |x: int| (x + 1) as int;
    lemma_set_map_insert::<int, int>(s, f, 0int);
    // s.insert(0).map(f) == {1}, but s.map(f) == {}
    assert(s.insert(0int).map(f) =~= s.map(f));
}

// Test 2: Use x instead of f(x) in the insert — wrong mapped value
// True postcondition: ... == s.map(f).insert(f(x))
// Mutated:            ... == s.map(f).insert(x)    [x instead of f(x)]
// SHOULD FAIL
proof fn test_mutation_wrong_insert_value()
{
    let s = Set::<int>::empty();
    let f = |x: int| (x + 10) as int;
    lemma_set_map_insert::<int, int>(s, f, 5int);
    // s.insert(5).map(f) == {15}
    // s.map(f).insert(5) == {5}   (wrong!)
    assert(s.insert(5int).map(f) =~= s.map(f).insert(5int));
}

// Test 3: Negate the postcondition entirely
// True postcondition: s.insert(x).map(f) == s.map(f).insert(f(x))
// Mutated:            s.insert(x).map(f) != s.map(f).insert(f(x))
// SHOULD FAIL
proof fn test_mutation_negated_postcondition()
{
    let s = Set::<int>::empty();
    let f = |x: int| x;
    lemma_set_map_insert::<int, int>(s, f, 0int);
    // The postcondition says they are equal; assert they are NOT equal
    assert(s.insert(0int).map(f) !== s.map(f).insert(f(0int)));
}

// Test 4: Apply f twice — use f(f(x)) instead of f(x) on the RHS
// True postcondition: ... == s.map(f).insert(f(x))
// Mutated:            ... == s.map(f).insert(f(f(x)))
// SHOULD FAIL
proof fn test_mutation_double_application()
{
    let s = Set::<int>::empty();
    let f = |x: int| (x + 1) as int;
    lemma_set_map_insert::<int, int>(s, f, 0int);
    // s.insert(0).map(f) == {1}
    // s.map(f).insert(f(f(0))) == {}.insert(f(1)) == {2}   (wrong!)
    assert(s.insert(0int).map(f) =~= s.map(f).insert(f(f(0int))));
}

}
