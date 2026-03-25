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
    pub fn init(&mut self)
        requires
            N > 2,
            N < SLLIndex::MAX,
            old(self).array_wf(),
        ensures
            self.wf(),
            self@ =~= Seq::empty(),
            self.len() == 0,
    {
        unimplemented!()
    }

}

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

    #[verifier::external_body]
    #[verifier(external_body)]
    pub fn set_value(&mut self, index: SLLIndex, v: Option<T>)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(self).arr_seq@[i],
            self.arr_seq@[index as int].prev == old(self).arr_seq@[index as int].prev,
            self.arr_seq@[index as int].next == old(self).arr_seq@[index as int].next,
            self.arr_seq@[index as int].value == v,
            self.spec_seq@ == old(self).spec_seq@,
            self.value_list@ == old(self).value_list@,
            self.free_list@ == old(self).free_list@,
            self.value_list_head == old(self).value_list_head,
            self.value_list_tail == old(self).value_list_tail,
            self.value_list_len == old(self).value_list_len,
            self.free_list_head == old(self).free_list_head,
            self.free_list_tail == old(self).free_list_tail,
            self.free_list_len == old(self).free_list_len,
            old(self).free_list_wf() ==> self.free_list_wf(),
            old(self).value_list_wf() ==> self.value_list_wf(),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    #[verifier(external_body)]
    pub fn set_next(&mut self, index: SLLIndex, v: SLLIndex)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(self).arr_seq@[i],
            self.arr_seq@[index as int].prev == old(self).arr_seq@[index as int].prev,
            self.arr_seq@[index as int].value == old(self).arr_seq@[index as int].value,
            self.arr_seq@[index as int].next == v,
            self.spec_seq@ == old(self).spec_seq@,
            self.value_list@ == old(self).value_list@,
            self.free_list@ == old(self).free_list@,
            self.value_list_head == old(self).value_list_head,
            self.value_list_tail == old(self).value_list_tail,
            self.value_list_len == old(self).value_list_len,
            self.free_list_head == old(self).free_list_head,
            self.free_list_tail == old(self).free_list_tail,
            self.free_list_len == old(self).free_list_len,
    {
        unimplemented!()
    }

    #[verifier::external_body]
    #[verifier(external_body)]
    pub fn set_prev(&mut self, index: SLLIndex, v: SLLIndex)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(self).arr_seq@[i],
            self.arr_seq@[index as int].next == old(self).arr_seq@[index as int].next,
            self.arr_seq@[index as int].value == old(self).arr_seq@[index as int].value,
            self.arr_seq@[index as int].prev == v,
            self.spec_seq@ == old(self).spec_seq@,
            self.value_list@ == old(self).value_list@,
            self.free_list@ == old(self).free_list@,
            self.value_list_head == old(self).value_list_head,
            self.value_list_tail == old(self).value_list_tail,
            self.value_list_len == old(self).value_list_len,
            self.free_list_head == old(self).free_list_head,
            self.free_list_tail == old(self).free_list_tail,
            self.free_list_len == old(self).free_list_len,
    {
        unimplemented!()
    }

}

// Helpers
#[verifier::external_body]
fn create_sll<T: Copy, const N: usize>() -> (sll: StaticLinkedList<T, N>)
    ensures sll.array_wf()
{
    unimplemented!()
}

// ===== Round 5: Cross-function Misuse & Edge Cases (should all FAIL) =====

// Test 1: After set_value, assert that next field changed (frame violation)
fn test_set_value_changes_next() {
    let mut sll = create_sll::<u64, 5>();
    sll.set_next(0i32, 3i32);
    sll.set_value(0i32, Some(42u64));
    assert(sll.arr_seq@[0int].next != 3i32);
}

// Test 2: After set_next, assert that value field changed (frame violation)
fn test_set_next_changes_value() {
    let mut sll = create_sll::<u64, 5>();
    sll.set_value(0i32, Some(42u64));
    sll.set_next(0i32, 3i32);
    assert(sll.arr_seq@[0int].value != Some(42u64));
}

// Test 3: After set_prev, assert that next field changed (frame violation)
fn test_set_prev_changes_next() {
    let mut sll = create_sll::<u64, 5>();
    sll.set_next(0i32, 3i32);
    sll.set_prev(0i32, 2i32);
    assert(sll.arr_seq@[0int].next != 3i32);
}

// Test 4: After set_value at index 0, assert node at index 1 changed (frame violation)
fn test_set_value_changes_other_index() {
    let mut sll = create_sll::<u64, 5>();
    sll.set_next(1i32, 4i32);
    sll.set_value(0i32, Some(42u64));
    assert(sll.arr_seq@[1int].next != 4i32);
}

// Test 5: After init, assert that calling len() without wf gives view length
// (tests that len == view.len is only guaranteed when wf holds)
proof fn test_len_without_wf_not_view_len(sll: StaticLinkedList<u64, 5>)
    requires
        !sll.wf(),
{
    // Without wf, we can't prove value_list_len == view.len()
    assert(sll.value_list_len as int == sll@.len());
}

}
