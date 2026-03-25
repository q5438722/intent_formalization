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

impl<T, const N: usize> StaticLinkedList<T, N> {

    pub open spec fn spec_len(&self) -> usize {
        self@.len() as usize
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (l: usize)
        ensures
            l == self.value_list_len,
            self.wf() ==> l == self.len(),
            self.wf() ==> l == self@.len(),
    {
        unimplemented!()
    }

    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int|
            #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self.len() && 0 <= j < self.len() && i != j ==> self.spec_seq@[i]
                != self.spec_seq@[j]
    }

    pub open spec fn view(&self) -> Seq<T> {
        self.spec_seq@
    }

    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex
        recommends
            self.wf(),
            self@.contains(v),
    {
        self.value_list@[self@.index_of(v)]
    }

    pub closed spec fn prev_free_node_of(&self, i: nat) -> int
        recommends
            i < self.free_list@.len(),
    {
        if i == 0 {
            -1
        } else {
            self.free_list@[i - 1int] as int
        }
    }

    pub closed spec fn next_free_node_of(&self, i: nat) -> int
        recommends
            i < self.free_list@.len(),
    {
        if i + 1 == self.free_list@.len() {
            -1
        } else {
            self.free_list@[i + 1int] as int
        }
    }

    pub closed spec fn wf_free_node_head(&self) -> bool {
        if self.free_list@.len() == 0 {
            self.free_list_head == -1
        } else {
            self.free_list_head == self.free_list@[0]
        }
    }

    pub closed spec fn wf_free_node_tail(&self) -> bool {
        if self.free_list@.len() == 0 {
            self.free_list_tail == -1
        } else {
            self.free_list_tail == self.free_list@[self.free_list@.len() - 1]
        }
    }

    pub closed spec fn free_list_wf(&self) -> bool {
        &&& forall|i: nat|
            #![trigger self.arr_seq@[self.free_list@[i as int] as int].next]
            #![trigger self.next_free_node_of(i)]
            0 <= i < self.free_list@.len() ==> self.arr_seq@[self.free_list@[i as int] as int].next
                == self.next_free_node_of(i)
        &&& forall|i: nat|
            #![trigger self.arr_seq@[self.free_list@[i as int] as int].prev]
            #![trigger self.prev_free_node_of(i)]
            0 <= i < self.free_list@.len() ==> self.arr_seq@[self.free_list@[i as int] as int].prev
                == self.prev_free_node_of(i)
        &&& forall|i: nat|
            #![trigger self.free_list@[i as int]]
            0 <= i < self.free_list@.len() ==> 0 <= self.free_list@[i as int] < N
        &&& forall|i: int, j: int|
            #![trigger self.free_list@[i], self.free_list@[j]]
            0 <= i < self.free_list_len && 0 <= j < self.free_list_len && i != j
                ==> self.free_list@[i] != self.free_list@[j]
        &&& self.wf_free_node_head()
        &&& self.wf_free_node_tail()
        &&& self.free_list_len == self.free_list@.len()
    }

    pub closed spec fn prev_value_node_of(&self, i: int) -> int
        recommends
            0 <= i < self.value_list@.len(),
    {
        if i == 0 {
            -1
        } else {
            self.value_list@[i - 1int] as int
        }
    }

    pub closed spec fn next_value_node_of(&self, i: int) -> int
        recommends
            0 <= i < self.value_list@.len(),
    {
        if i + 1 == self.value_list@.len() {
            -1
        } else {
            self.value_list@[i + 1int] as int
        }
    }

    pub closed spec fn wf_value_node_head(&self) -> bool {
        if self.value_list@.len() == 0 {
            self.value_list_head == -1
        } else {
            self.value_list_head == self.value_list@[0]
        }
    }

    pub closed spec fn wf_value_node_tail(&self) -> bool {
        if self.value_list@.len() == 0 {
            self.value_list_tail == -1
        } else {
            self.value_list_tail == self.value_list@[self.value_list@.len() - 1]
        }
    }

    pub closed spec fn value_list_wf(&self) -> bool {
        &&& forall|i: int|
            #![trigger self.arr_seq@[self.value_list@[i as int] as int].next]
            #![trigger self.next_value_node_of(i)]
            0 <= i < self.value_list@.len()
                ==> self.arr_seq@[self.value_list@[i as int] as int].next
                == self.next_value_node_of(i)
        &&& forall|i: int|
            #![trigger self.arr_seq@[self.value_list@[i as int] as int].prev]
            #![trigger self.prev_value_node_of(i)]
            0 <= i < self.value_list@.len()
                ==> self.arr_seq@[self.value_list@[i as int] as int].prev
                == self.prev_value_node_of(i)
        &&& forall|i: int|
            #![trigger self.value_list@[i as int]]
            0 <= i < self.value_list@.len() ==> 0 <= self.value_list@[i as int] < N
        &&& self.unique()
        &&& self.wf_value_node_head()
        &&& self.wf_value_node_tail()
        &&& self.value_list_len == self.value_list@.len()
    }

    pub closed spec fn array_wf(&self) -> bool {
        &&& self.arr_seq@.len() == N
        &&& self.size == N
    }

    pub closed spec fn spec_seq_wf(&self) -> bool {
        &&& self.spec_seq@.len() == self.value_list_len
        &&& forall|i: int|
            #![trigger self.spec_seq@[i as int]]
            #![trigger self.value_list@[i as int]]
            0 <= i < self.value_list_len
                ==> self.arr_seq@[self.value_list@[i as int] as int].value.is_Some()
                && self.arr_seq@[self.value_list@[i as int] as int].value.get_Some_0()
                =~= self.spec_seq@[i as int]
    }

    pub closed spec fn wf(&self) -> bool {
        &&& N <= i32::MAX
        &&& N > 2
        &&& self.array_wf()
        &&& self.free_list_len + self.value_list_len == N
        &&& self.value_list_wf()
        &&& self.free_list_wf()
        &&& self.spec_seq_wf()
        &&& forall|i: int, j: int|
            #![trigger self.value_list@[i], self.free_list@[j]]
            0 <= i < self.value_list@.len() && 0 <= j < self.free_list@.len()
                ==> self.value_list@[i] != self.free_list@[j]
    }

}


// ==================== LOGICAL TESTS ====================

// Logical Test 1: All old values are preserved after removal (false: removed value is gone)
// Tests if the spec admits the unintended property that removal preserves all elements
// SHOULD FAIL
proof fn test_logical_all_values_preserved()
{
    let old_sll: StaticLinkedList<u64, 10>;
    let new_sll: StaticLinkedList<u64, 10>;
    let ret: u64;
    let remove_index: SLLIndex;
    let v: u64;

    // Assume preconditions
    assume(old_sll.wf());
    assume(old_sll@.contains(v));
    assume(old_sll.get_node_ref(v) == remove_index);
    assume(old_sll.value_list_len != 1);
    assume(old_sll.free_list_len != 0);
    assume(old_sll.value_list_head == remove_index);

    // Assume postconditions
    assume(new_sll.wf());
    assume(new_sll@.len() + 1 == old_sll@.len());
    assume(new_sll.unique());
    assume(new_sll@ =~= old_sll@.remove_value(ret));
    assume(ret == v);

    // SHOULD FAIL: not all values are preserved; ret is removed
    assert(forall|x: u64| old_sll@.contains(x) ==> new_sll@.contains(x));
}

// Logical Test 2: The new list is always empty after removal (too strong)
// Tests if the spec allows concluding the result is always empty
// SHOULD FAIL
proof fn test_logical_result_always_empty()
{
    let old_sll: StaticLinkedList<u64, 10>;
    let new_sll: StaticLinkedList<u64, 10>;
    let ret: u64;
    let remove_index: SLLIndex;
    let v: u64;

    assume(old_sll.wf());
    assume(old_sll@.contains(v));
    assume(old_sll.get_node_ref(v) == remove_index);
    assume(old_sll.value_list_len != 1);
    assume(old_sll.free_list_len != 0);
    assume(old_sll.value_list_head == remove_index);

    assume(new_sll.wf());
    assume(new_sll@.len() + 1 == old_sll@.len());
    assume(new_sll.unique());
    assume(new_sll@ =~= old_sll@.remove_value(ret));
    assume(ret == v);

    // SHOULD FAIL: old has at least 2 elements (value_list_len != 1), so new has >= 1
    assert(new_sll@.len() == 0);
}

// Logical Test 3: Node references change for remaining elements (spec says they don't)
// Tests the negation of the node-ref preservation postcondition
// SHOULD FAIL
proof fn test_logical_node_refs_change()
{
    let old_sll: StaticLinkedList<u64, 10>;
    let new_sll: StaticLinkedList<u64, 10>;
    let ret: u64;
    let remove_index: SLLIndex;
    let v: u64;
    let w: u64;

    assume(old_sll.wf());
    assume(old_sll@.contains(v));
    assume(old_sll.get_node_ref(v) == remove_index);
    assume(old_sll.value_list_len != 1);
    assume(old_sll.free_list_len != 0);
    assume(old_sll.value_list_head == remove_index);

    assume(new_sll.wf());
    assume(new_sll@.len() + 1 == old_sll@.len());
    assume(new_sll.unique());
    assume(new_sll@ =~= old_sll@.remove_value(ret));
    assume(ret == v);
    // Assume node-ref preservation postcondition
    assume(forall|u: u64| #![auto] new_sll@.contains(u) ==> old_sll.get_node_ref(u) == new_sll.get_node_ref(u));

    // Assume w is a remaining element
    assume(new_sll@.contains(w));

    // SHOULD FAIL: node refs are preserved for remaining elements
    assert(old_sll.get_node_ref(w) != new_sll.get_node_ref(w));
}

// Logical Test 4: The first element of new list is always a specific constant
// Tests if the spec over-constrains the first element to a constant
// SHOULD FAIL
proof fn test_logical_first_element_constant()
{
    let old_sll: StaticLinkedList<u64, 10>;
    let new_sll: StaticLinkedList<u64, 10>;
    let ret: u64;
    let remove_index: SLLIndex;
    let v: u64;

    assume(old_sll.wf());
    assume(old_sll@.contains(v));
    assume(old_sll.get_node_ref(v) == remove_index);
    assume(old_sll.value_list_len != 1);
    assume(old_sll.free_list_len != 0);
    assume(old_sll.value_list_head == remove_index);

    assume(new_sll.wf());
    assume(new_sll@.len() + 1 == old_sll@.len());
    assume(new_sll.unique());
    assume(new_sll@ =~= old_sll@.remove_value(ret));
    assume(ret == v);

    // SHOULD FAIL: first element of new list is not always 0
    assert(new_sll@.len() > 0 ==> new_sll@[0] == 0u64);
}

// Logical Test 5: Removal is idempotent (removing twice from same state gives same result)
// Tests a structural/global assumption about determinism of sequential removals
// SHOULD FAIL
proof fn test_logical_double_removal_same()
{
    let old_sll: StaticLinkedList<u64, 10>;
    let new_sll1: StaticLinkedList<u64, 10>;
    let new_sll2: StaticLinkedList<u64, 10>;
    let ret1: u64;
    let ret2: u64;
    let remove_index: SLLIndex;
    let v: u64;

    // Same preconditions for both removals
    assume(old_sll.wf());
    assume(old_sll@.contains(v));
    assume(old_sll.get_node_ref(v) == remove_index);
    assume(old_sll.value_list_len != 1);
    assume(old_sll.free_list_len != 0);
    assume(old_sll.value_list_head == remove_index);

    // Postconditions for first removal
    assume(new_sll1.wf());
    assume(new_sll1@.len() + 1 == old_sll@.len());
    assume(new_sll1@ =~= old_sll@.remove_value(ret1));
    assume(ret1 == v);

    // Postconditions for second removal (different result)
    assume(new_sll2.wf());
    assume(new_sll2@.len() + 1 == old_sll@.len());
    assume(new_sll2@ =~= old_sll@.remove_value(ret2));
    assume(ret2 == v);

    // Sequences should be equal (deterministic), but internal state may differ
    // SHOULD FAIL: can't prove internal state equality from spec alone
    assert(new_sll1.value_list_head == new_sll2.value_list_head);
}

}
