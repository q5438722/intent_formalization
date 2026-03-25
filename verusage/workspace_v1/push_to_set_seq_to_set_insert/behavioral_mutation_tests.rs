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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// ============================================================

// SHOULD FAIL: Mutation — omit the insert, claim push-to-set equals original set
proof fn test_mutation_1_push_set_eq_original_set()
{
    let s = Seq::<int>::empty();
    let e: int = 42;
    push_to_set_eq_to_set_insert(s, e);
    // Mutated: should be s.to_set().insert(e), not s.to_set()
    assert(s.push(e).to_set() =~= s.to_set());
}

// SHOULD FAIL: Mutation — remove instead of insert
proof fn test_mutation_2_push_set_eq_remove()
{
    let s = Seq::<int>::empty().push(1int).push(2int);
    let e: int = 3;
    push_to_set_eq_to_set_insert(s, e);
    // Mutated: remove(e) instead of insert(e)
    assert(s.push(e).to_set() =~= s.to_set().remove(e));
}

// SHOULD FAIL: Mutation — negate containment of pushed element
proof fn test_mutation_3_pushed_element_not_in_set()
{
    let s = Seq::<int>::empty().push(10int);
    let e: int = 20;
    push_to_set_eq_to_set_insert(s, e);
    // Pushed element must be in the set
    assert(!s.push(e).to_set().contains(e));
}

// SHOULD FAIL: Mutation — insert a different element
proof fn test_mutation_4_insert_wrong_element()
{
    let s = Seq::<int>::empty();
    let e: int = 5;
    push_to_set_eq_to_set_insert(s, e);
    // Mutated: insert 99 instead of 5
    assert(s.push(e).to_set() =~= s.to_set().insert(99int));
}

// SHOULD FAIL: Mutation — claim the sets are not equal (negate the ensures)
proof fn test_mutation_5_negate_ensures()
{
    let s = Seq::<int>::empty().push(1int);
    let e: int = 2;
    push_to_set_eq_to_set_insert(s, e);
    // The ensures says they ARE equal, so negation should fail
    assert(s.push(e).to_set() !== s.to_set().insert(e));
}

}
