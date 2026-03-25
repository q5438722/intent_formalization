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
// BOUNDARY TESTS: Edge cases that should be rejected
// ============================================================

// SHOULD FAIL: Pushing onto empty seq should NOT yield empty set
proof fn test_boundary_1_push_empty_gives_empty_set()
{
    let s = Seq::<int>::empty();
    let e: int = 0;
    push_to_set_eq_to_set_insert(s, e);
    // s.push(0).to_set() == {0}, not empty
    assert(s.push(e).to_set() =~= Set::empty());
}

// SHOULD FAIL: Push does NOT preserve sequence length
proof fn test_boundary_2_push_preserves_length()
{
    let s = Seq::<int>::empty();
    let e: int = 1;
    // push adds one element, so len increases
    assert(s.push(e).len() == s.len());
}

// SHOULD FAIL: After push, the element at index 0 is NOT absent
proof fn test_boundary_3_push_empty_seq_has_no_elements()
{
    let s = Seq::<int>::empty();
    let e: int = 7;
    // s.push(7) has length 1 with element 7 at index 0
    assert(!s.push(e).contains(e));
}

// SHOULD FAIL: Pushing zero onto a seq of length 0 does not yield set of len 0
proof fn test_boundary_4_push_zero_set_len_zero()
{
    let s = Seq::<int>::empty();
    let e: int = 0;
    push_to_set_eq_to_set_insert(s, e);
    assert(s.push(e).to_set().len() == 0);
}

// SHOULD FAIL: to_set of single-element seq is not equal to set with different element
proof fn test_boundary_5_push_gives_wrong_singleton()
{
    let s = Seq::<int>::empty();
    let e: int = 1;
    push_to_set_eq_to_set_insert(s, e);
    // The set should be {1}, not {2}
    assert(s.push(e).to_set() =~= Set::empty().insert(2int));
}

}
