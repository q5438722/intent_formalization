use vstd::prelude::*;

fn main() {}

verus! {

// Original lemma under test
pub proof fn push_to_set_eq_to_set_insert<A>(s: Seq<A>, e: A)
    ensures s.push(e).to_set() == s.to_set().insert(e)
{
    assert(s.push(e).to_set() =~= s.to_set().insert(e)) by {
        assert forall |obj: A| s.push(e).to_set().contains(obj) implies #[trigger] s.to_set().insert(e).contains(obj) by {
            assert(s.push(e).contains(obj));
            if obj == e {
                assert(s.to_set().insert(e).contains(e));
            } else {
                assert(s.contains(obj));
                assert(s.to_set().contains(obj));
                assert(s.to_set().insert(e).contains(obj));
            }
        }
        assert forall |obj: A| s.to_set().insert(e).contains(obj) implies #[trigger] s.push(e).to_set().contains(obj) by {
            if obj == e {
                assert(s.push(e).last() == e);
                assert(s.push(e).contains(e));
            } else {
                assert(s.to_set().contains(obj));
                assert(s.contains(obj));
                assert(s == s.push(e).drop_last());
            }
        }
    }
}

// ============================================================
// LOGICAL TESTS: Properties NOT guaranteed by the spec
// ============================================================

// SHOULD FAIL: Stronger property — set cardinality always increases by 1
// This is false when e is already in s
proof fn test_logical_1_set_size_always_increases()
{
    let s = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let e: int = 1;
    push_to_set_eq_to_set_insert(s, e);
    // 1 is already in s, so to_set().insert(1) doesn't increase cardinality
    assert(s.push(e).to_set().len() == s.to_set().len() + 1);
}

// SHOULD FAIL: Push is reversible via remove — false when e was already in s
proof fn test_logical_2_push_reversible_via_remove()
{
    let s = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let e: int = 1;
    push_to_set_eq_to_set_insert(s, e);
    // Removing e from s.push(e).to_set() should give s.to_set()
    // But s already contains 1, so removing 1 from {1,2} gives {2}, not {1,2}
    assert(s.push(e).to_set().remove(e) =~= s.to_set());
}

// SHOULD FAIL: Sequence length equals set cardinality after push
// False when sequence has duplicates
proof fn test_logical_3_seq_len_eq_set_len()
{
    let s = Seq::<int>::empty().push(1int).push(1int);
    let e: int = 2;
    push_to_set_eq_to_set_insert(s, e);
    // s.push(e) has len 3, but to_set() has {1, 2} with len 2
    assert(s.push(e).to_set().len() == s.push(e).len());
}

// SHOULD FAIL: Pushing two different elements gives sets of different cardinality
// False when both elements are already in s
proof fn test_logical_4_different_pushes_different_cardinality()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let e1: int = 1;
    let e2: int = 2;
    push_to_set_eq_to_set_insert(s, e1);
    push_to_set_eq_to_set_insert(s, e2);
    // Both pushes result in {1, 2}, same cardinality
    assert(s.push(e1).to_set().len() != s.push(e2).to_set().len());
}

// SHOULD FAIL: to_set is injective (different seqs give different sets)
// False: [1,2,1] and [1,2] have the same to_set
proof fn test_logical_5_to_set_injective()
{
    let s1 = Seq::<int>::empty().push(1int).push(2int).push(1int);
    let s2 = Seq::<int>::empty().push(1int).push(2int);
    let e: int = 3;
    push_to_set_eq_to_set_insert(s1, e);
    push_to_set_eq_to_set_insert(s2, e);
    // Both s1.push(3).to_set() and s2.push(3).to_set() equal {1,2,3}
    // Claiming they differ should fail
    assert(s1.push(e).to_set() !== s2.push(e).to_set());
}

}
