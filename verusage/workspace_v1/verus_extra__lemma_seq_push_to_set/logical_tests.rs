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
// LOGICAL TESTS: Properties NOT guaranteed by the spec
// ============================================================

// SHOULD FAIL: Stronger property — set cardinality always increases by 1
// False when x is already in s (duplicate element)
proof fn test_logical_1_set_size_always_increases()
{
    let s = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let x: int = 1;
    lemma_seq_push_to_set(s, x);
    // 1 is already in s, so to_set().insert(1) doesn't increase cardinality
    assert(s.push(x).to_set().len() == s.to_set().len() + 1);
}

// SHOULD FAIL: Push is reversible via remove — false when x was already in s
proof fn test_logical_2_push_reversible_via_remove()
{
    let s = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let x: int = 1;
    lemma_seq_push_to_set(s, x);
    // Removing x from s.push(x).to_set() should give s.to_set()
    // But s already contains 1, so removing 1 from {1,2} gives {2}, not {1,2}
    assert(s.push(x).to_set().remove(x) =~= s.to_set());
}

// SHOULD FAIL: Sequence length equals set cardinality after push
// False when sequence has duplicates
proof fn test_logical_3_seq_len_eq_set_len()
{
    let s = Seq::<int>::empty().push(1int).push(1int);
    let x: int = 2;
    lemma_seq_push_to_set(s, x);
    // s.push(x) has len 3, but to_set() has {1, 2} with len 2
    assert(s.push(x).to_set().len() == s.push(x).len());
}

// SHOULD FAIL: to_set is injective — different seqs give different sets
// False: [1,2,1] and [1,2] have the same to_set
proof fn test_logical_4_to_set_injective()
{
    let s1 = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let s2 = Seq::<int>::empty().push(1int).push(2int);
    let x: int = 3;
    lemma_seq_push_to_set(s1, x);
    lemma_seq_push_to_set(s2, x);
    // Both give {1,2,3}, claiming they differ should fail
    assert(s1.push(x).to_set() !== s2.push(x).to_set());
}

// SHOULD FAIL: Double push gives strictly larger set than single push
// False when x is already in s (second push adds nothing new)
proof fn test_logical_5_double_push_strictly_larger()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let x: int = 1;
    lemma_seq_push_to_set(s, x);
    lemma_seq_push_to_set(s.push(x), x);
    // s.push(x).push(x).to_set() == {1,2}, same as s.push(x).to_set()
    // Claiming strictly more elements should fail
    assert(s.push(x).push(x).to_set().len() > s.push(x).to_set().len());
}

}
