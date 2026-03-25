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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// ============================================================

// SHOULD FAIL: Mutation — omit insert, claim push-to-set equals original set
proof fn test_mutation_1_push_set_eq_original_set()
{
    let s = Seq::<int>::empty();
    let x: int = 10;
    lemma_seq_push_to_set(s, x);
    // Mutated: should be s.to_set().insert(x), not s.to_set()
    assert(s.push(x).to_set() =~= s.to_set());
}

// SHOULD FAIL: Mutation — remove instead of insert
proof fn test_mutation_2_push_set_eq_remove()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let x: int = 3;
    lemma_seq_push_to_set(s, x);
    // Mutated: remove(x) instead of insert(x)
    assert(s.push(x).to_set() =~= s.to_set().remove(x));
}

// SHOULD FAIL: Mutation — negate containment of pushed element
proof fn test_mutation_3_pushed_element_not_in_set()
{
    let s = Seq::<int>::empty().push(5int);
    let x: int = 20;
    lemma_seq_push_to_set(s, x);
    // Pushed element must be in the resulting set
    assert(!s.push(x).to_set().contains(x));
}

// SHOULD FAIL: Mutation — insert a different element than what was pushed
proof fn test_mutation_4_insert_wrong_element()
{
    let s = Seq::<int>::empty();
    let x: int = 5;
    lemma_seq_push_to_set(s, x);
    // Mutated: insert 77 instead of 5
    assert(s.push(x).to_set() =~= s.to_set().insert(77int));
}

// SHOULD FAIL: Mutation — negate the ensures (claim sets are NOT equal)
proof fn test_mutation_5_negate_ensures()
{
    let s = Seq::<int>::empty().push(1int);
    let x: int = 2;
    lemma_seq_push_to_set(s, x);
    // The ensures says they ARE equal, so negation should fail
    assert(s.push(x).to_set() !== s.to_set().insert(x));
}

}
