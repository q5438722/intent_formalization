use vstd::prelude::*;

fn main() {}

verus!{

pub type SLLIndex = i32;

#[derive(Debug)]
pub struct Node<T> {
    pub value: Option<T>,
    pub next: SLLIndex,
    pub prev: SLLIndex,
}

#[verifier::reject_recursive_types(T)]
pub struct StaticLinkedList<T, const N: usize> {
    pub ar: [Node<T>; N],
    pub spec_seq: Ghost<Seq<T>>,
    pub value_list: Ghost<Seq<SLLIndex>>,
    pub value_list_head: SLLIndex,
    pub value_list_tail: SLLIndex,
    pub value_list_len: usize,
    pub free_list: Ghost<Seq<SLLIndex>>,
    pub free_list_head: SLLIndex,
    pub free_list_tail: SLLIndex,
    pub free_list_len: usize,
    pub size: usize,
    pub arr_seq: Ghost<Seq<Node<T>>>,
}

impl<T, const N: usize> View for StaticLinkedList<T, N> {
    type V = Seq<T>;
    open spec fn view(&self) -> Seq<T> {
        self.spec_seq@
    }
}

impl<T, const N: usize> StaticLinkedList<T, N> {
    pub open spec fn spec_len(&self) -> usize {
        self@.len() as usize
    }

    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int|
            #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self@.len() && 0 <= j < self@.len() && i != j ==> self.spec_seq@[i]
                != self.spec_seq@[j]
    }
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests for determinism, stronger bounds, structural assumptions,
// and cross-function misuse. All tests SHOULD FAIL verification.

// Test 1: Determinism — two structures that both satisfy init's
// postconditions (empty view, len 0) are NOT guaranteed to have
// identical internal state. Claim: free_list_head is equal.
// SHOULD FAIL
proof fn test_logical_determinism_free_head(
    sll1: StaticLinkedList<u64, 4>,
    sll2: StaticLinkedList<u64, 4>,
)
    requires
        sll1@ =~= Seq::<u64>::empty(),
        sll2@ =~= Seq::<u64>::empty(),
        sll1.value_list_len == 0,
        sll2.value_list_len == 0,
{
    assert(sll1.free_list_head == sll2.free_list_head);
}

// Test 2: value_list@.len() == 0 is NOT derivable from just
// self@ =~= Seq::empty(). value_list is a separate ghost field.
// SHOULD FAIL
proof fn test_logical_value_list_len_from_view(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll@ =~= Seq::<u64>::empty(),
{
    assert(sll.value_list@.len() == 0);
}

// Test 3: From value_list_len == 0 alone, we cannot derive
// free_list_len == N without the wf() invariant (which is closed).
// SHOULD FAIL
proof fn test_logical_free_list_len_from_value_len(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll.value_list_len == 0,
{
    assert(sll.free_list_len == 4);
}

// Test 4: After init, all nodes should have value == None
// (set by the loop). But this is NOT in init's ensures clause.
// SHOULD FAIL
proof fn test_logical_all_values_none(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll@ =~= Seq::<u64>::empty(),
        sll.value_list_len == 0,
{
    assert(forall|i: int| 0 <= i < 4 ==> (#[trigger] sll.arr_seq@[i]).value.is_None());
}

// Test 5: After init, free_list@[0] == 0 (from the loop).
// But this is an internal detail NOT exposed in ensures.
// SHOULD FAIL
proof fn test_logical_free_list_first_element(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll@ =~= Seq::<u64>::empty(),
        sll.value_list_len == 0,
{
    assert(sll.free_list@.len() > 0 && sll.free_list@[0int] == 0);
}

// Test 6: spec_len() == self@.len() as usize. Without knowing
// self@ is empty (only knowing value_list_len == 0), we cannot
// derive spec_len() == 0 since they are independent fields.
// SHOULD FAIL
proof fn test_logical_spec_len_from_value_list_len(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll.value_list_len == 0,
{
    assert(sll.spec_len() == 0);
}

// Test 7: Two structures with the same view are NOT guaranteed
// to have the same value_list_len. The view (spec_seq) and
// value_list_len are independently stored fields.
// SHOULD FAIL
proof fn test_logical_same_view_same_len(
    sll1: StaticLinkedList<u64, 4>,
    sll2: StaticLinkedList<u64, 4>,
)
    requires
        sll1@ =~= sll2@,
{
    assert(sll1.value_list_len == sll2.value_list_len);
}

// Test 8: set_value preserves free_list@ (the ghost sequence).
// But set_value says nothing about free_list@ entries being
// valid indices. Claim: all free_list entries are in [0, N).
// SHOULD FAIL
proof fn test_logical_set_value_free_indices_valid(
    free_list_seq: Seq<SLLIndex>,
)
    requires
        free_list_seq.len() > 0,
{
    assert(forall|i: int| 0 <= i < free_list_seq.len() ==>
        0 <= #[trigger] free_list_seq[i] && (free_list_seq[i] as usize) < 4);
}

// Test 9: Two structures satisfying init's postconditions are
// NOT guaranteed to have the same free_list_tail.
// SHOULD FAIL
proof fn test_logical_init_unique_free_tail(
    sll1: StaticLinkedList<u64, 4>,
    sll2: StaticLinkedList<u64, 4>,
)
    requires
        sll1@ =~= Seq::<u64>::empty(),
        sll2@ =~= Seq::<u64>::empty(),
        sll1.value_list_len == 0,
        sll2.value_list_len == 0,
{
    assert(sll1.free_list_tail == sll2.free_list_tail);
}

// Test 10: After init, free_list_head >= 0 should hold (since
// the free list is non-empty). But this is NOT derivable from
// the closed wf() predicate or the ensures clause.
// SHOULD FAIL
proof fn test_logical_free_list_head_nonneg(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll@ =~= Seq::<u64>::empty(),
        sll.value_list_len == 0,
{
    assert(sll.free_list_head >= 0);
}

// Test 11: unique() is vacuously true for empty sequences.
// Claiming !unique() after init should be unprovable.
// SHOULD FAIL
proof fn test_logical_not_unique_after_init(
    sll: StaticLinkedList<u64, 4>,
)
    requires
        sll@ =~= Seq::<u64>::empty(),
{
    assert(!sll.unique());
}

// Test 12: An unconstrained SLLIndex prev value is NOT guaranteed
// to be >= -1. i32 can hold values down to i32::MIN.
// Claim: any prev is >= -1 without constraints.
// SHOULD FAIL
proof fn test_logical_prev_always_ge_neg1(
    prev_value: SLLIndex,
) {
    assert(prev_value >= -1);
}

}
