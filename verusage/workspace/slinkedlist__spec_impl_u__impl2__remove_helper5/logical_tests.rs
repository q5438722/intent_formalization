use vstd::prelude::*;

fn main() {}

verus! {

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
        recommends i < self.free_list@.len(),
    {
        if i == 0 { -1 } else { self.free_list@[i - 1int] as int }
    }

    pub closed spec fn next_free_node_of(&self, i: nat) -> int
        recommends i < self.free_list@.len(),
    {
        if i + 1 == self.free_list@.len() { -1 } else { self.free_list@[i + 1int] as int }
    }

    pub closed spec fn wf_free_node_head(&self) -> bool {
        if self.free_list@.len() == 0 { self.free_list_head == -1 }
        else { self.free_list_head == self.free_list@[0] }
    }

    pub closed spec fn wf_free_node_tail(&self) -> bool {
        if self.free_list@.len() == 0 { self.free_list_tail == -1 }
        else { self.free_list_tail == self.free_list@[self.free_list@.len() - 1] }
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
        recommends 0 <= i < self.value_list@.len(),
    {
        if i == 0 { -1 } else { self.value_list@[i - 1int] as int }
    }

    pub closed spec fn next_value_node_of(&self, i: int) -> int
        recommends 0 <= i < self.value_list@.len(),
    {
        if i + 1 == self.value_list@.len() { -1 } else { self.value_list@[i + 1int] as int }
    }

    pub closed spec fn wf_value_node_head(&self) -> bool {
        if self.value_list@.len() == 0 { self.value_list_head == -1 }
        else { self.value_list_head == self.value_list@[0] }
    }

    pub closed spec fn wf_value_node_tail(&self) -> bool {
        if self.value_list@.len() == 0 { self.value_list_tail == -1 }
        else { self.value_list_tail == self.value_list@[self.value_list@.len() - 1] }
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

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

    #[verifier::external_body]
    pub fn remove_helper5(&mut self, remove_index: SLLIndex, v: Ghost<T>) -> (ret: T)
        requires
            old(self).wf(),
            old(self)@.contains(v@),
            old(self).get_node_ref(v@) == remove_index,
            old(self).value_list_len != 1,
            old(self).free_list_len != 0 && old(self).value_list_head == remove_index && old(
                self,
            ).value_list_len != 1,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            self.unique(),
            self@ =~= old(self)@.remove_value(ret),
            ret == v@,
            forall|v:T|
                #![auto]
                self@.contains(v) ==> 
                    old(self).get_node_ref(v) == 
                        self.get_node_ref(v),
    {
        unimplemented!()
    }
}

// ==================== LOGICAL TESTS ====================

// SHOULD FAIL: remove_index is not guaranteed to be 0;
// it depends on internal allocation tracked by get_node_ref (closed spec)
fn test_remove_index_is_zero(sll: &mut StaticLinkedList<u64, 4>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0 && old(sll).value_list_head == remove_index && old(sll).value_list_len != 1,
{
    let _ = sll.remove_helper5(remove_index, v);
    assert(remove_index == 0); // SHOULD FAIL: remove_index can be any valid array index
}

// SHOULD FAIL: spec does NOT publicly expose that free_list_len changes;
// wf() is closed, so the internal invariant free_list_len + value_list_len == N is hidden
fn test_free_list_len_unchanged(sll: &mut StaticLinkedList<u64, 4>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0 && old(sll).value_list_head == remove_index && old(sll).value_list_len != 1,
{
    let old_free_len = sll.free_list_len;
    let _ = sll.remove_helper5(remove_index, v);
    assert(sll.free_list_len == old_free_len); // SHOULD FAIL: free_list_len should increase
}

// SHOULD FAIL: no ordering guarantee exists in the spec;
// the removed value is NOT guaranteed to be <= remaining elements
fn test_removed_leq_new_head(sll: &mut StaticLinkedList<u64, 4>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0 && old(sll).value_list_head == remove_index && old(sll).value_list_len != 1,
{
    let ret = sll.remove_helper5(remove_index, v);
    assert(ret <= sll@[0]); // SHOULD FAIL: spec has no ordering invariant
}

// SHOULD FAIL: value_list_head is not constrained by public postconditions
fn test_value_list_head_unchanged(sll: &mut StaticLinkedList<u64, 4>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0 && old(sll).value_list_head == remove_index && old(sll).value_list_len != 1,
{
    let old_head = sll.value_list_head;
    let _ = sll.remove_helper5(remove_index, v);
    assert(sll.value_list_head == old_head); // SHOULD FAIL: head should change after head removal
}

// SHOULD FAIL: after removing from a 2-element list, len becomes 1;
// asserting len >= 2 is not always true
fn test_stronger_length_bound(sll: &mut StaticLinkedList<u64, 4>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0 && old(sll).value_list_head == remove_index && old(sll).value_list_len != 1,
{
    let _ = sll.remove_helper5(remove_index, v);
    assert(sll.len() >= 2); // SHOULD FAIL: old len could be 2, making new len 1
}

} // end verus!
