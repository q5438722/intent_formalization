use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// Original lemma under test
#[verifier::spinoff_prover]
pub proof fn lemma_seq_push_to_set<A>(s: Seq<A>, x: A)
    ensures s.push(x).to_set() == s.to_set().insert(x)
{
    assert_sets_equal!(s.push(x).to_set() == s.to_set().insert(x), elem => {
        if elem == x {
            assert(s.push(x)[s.len() as int] == x);
            assert(s.push(x).contains(x))
        } else {
            if s.to_set().insert(x).contains(elem) {
                assert(s.to_set().contains(elem));
                let i = choose |i: int| 0 <= i < s.len() && s[i] == elem;
                assert(s.push(x)[i] == elem);
            }
        }
    });
}

// ============================================================
// BOUNDARY TESTS: Edge cases that should be rejected
// ============================================================

// SHOULD FAIL: Pushing onto empty seq should NOT yield empty set
proof fn test_boundary_1_push_empty_gives_empty_set()
{
    let s = Seq::<int>::empty();
    let x: int = 42;
    lemma_seq_push_to_set(s, x);
    // s.push(42).to_set() == {42}, not empty
    assert(s.push(x).to_set() =~= Set::<int>::empty());
}

// SHOULD FAIL: Push does NOT preserve sequence length (it increases by 1)
proof fn test_boundary_2_push_preserves_seq_length()
{
    let s = Seq::<int>::empty();
    let x: int = 7;
    // s.push(x).len() == 1, not 0
    assert(s.push(x).len() == s.len());
}

// SHOULD FAIL: Pushed element is NOT contained in the resulting set
proof fn test_boundary_3_pushed_element_not_in_result()
{
    let s = Seq::<int>::empty();
    let x: int = 99;
    lemma_seq_push_to_set(s, x);
    // x must be in s.push(x).to_set()
    assert(!s.push(x).to_set().contains(x));
}

// SHOULD FAIL: Pushing onto empty seq does NOT produce a set of cardinality 0
proof fn test_boundary_4_push_empty_set_cardinality_zero()
{
    let s = Seq::<int>::empty();
    let x: int = 0;
    lemma_seq_push_to_set(s, x);
    // s.push(0).to_set() == {0} has cardinality 1, not 0
    assert(s.push(x).to_set().len() == 0);
}

// SHOULD FAIL: Pushing x gives a set containing only a different value y
proof fn test_boundary_5_push_gives_wrong_singleton()
{
    let s = Seq::<int>::empty();
    let x: int = 1;
    lemma_seq_push_to_set(s, x);
    // s.push(1).to_set() == {1}, NOT {2}
    assert(s.push(x).to_set() =~= Set::<int>::empty().insert(2int));
}

}
