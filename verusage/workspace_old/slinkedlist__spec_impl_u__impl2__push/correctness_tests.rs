use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions ===
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

// === Spec functions ===
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

// === Push function (external_body — test spec interface only) ===
impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

    #[verifier::external_body]
    pub fn push(&mut self, new_value: &T) -> (free_node_index: SLLIndex)
        requires
            old(self).wf(),
            old(self).len() < N,
            old(self).unique(),
            old(self)@.contains(*new_value) == false,
            N > 2,
        ensures
            self.wf(),
            self@ == old(self)@.push(*new_value),
            self.len() == old(self).len() + 1,
            forall|v:T|
                #![auto]
                old(self)@.contains(v) ==>
                    old(self).get_node_ref(v) ==
                        self.get_node_ref(v),
            self.get_node_ref(*new_value) == free_node_index,
            self.unique(),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    pub fn set_value(&mut self, index: SLLIndex, v: Option<T>)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(
                    self,
                ).arr_seq@[i],
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
    pub fn set_next(&mut self, index: SLLIndex, v: SLLIndex)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(
                    self,
                ).arr_seq@[i],
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
    pub fn set_prev(&mut self, index: SLLIndex, v: SLLIndex)
        requires
            old(self).array_wf(),
        ensures
            self.array_wf(),
            forall|i: int|
                #![trigger self.arr_seq@[i]]
                #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(
                    self,
                ).arr_seq@[i],
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

    #[verifier::external_body]
    pub fn get_next(&self, index: SLLIndex) -> (next: SLLIndex)
        requires
            0 <= index < N,
            self.array_wf(),
        ensures
            next == self.arr_seq@[index as int].next,
    {
        unimplemented!()
    }

}

// === Proof lemmas ===
#[verifier::external_body]
pub proof fn seq_push_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A|
            s.contains(x) ==> s.push(v).contains(v) && s.push(v).contains(x),
        forall|s: Seq<A>, v: A| #![auto] s.push(v).contains(v),
        forall|s: Seq<A>, v: A, x: A| !s.contains(x) && v != x ==> !s.push(v).contains(x),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn seq_push_index_of_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A|
            s.no_duplicates() && s.contains(v) && v != x
            ==>
            s.push(x).index_of(v) == s.index_of(v),
{
    unimplemented!()
}



// ============================
// === CORRECTNESS TESTS ===
// ============================

// Test 1: Push preserves well-formedness
fn test_push_preserves_wf(list: &mut StaticLinkedList<u64, 4>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let _idx = list.push(new_value);
    assert(list.wf());
}

// Test 2: Push appends to logical sequence
fn test_push_appends(list: &mut StaticLinkedList<u64, 4>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let ghost old_view = list@;
    let _idx = list.push(new_value);
    assert(list@ == old_view.push(*new_value));
}

// Test 3: Push increments length by 1
fn test_push_increments_len(list: &mut StaticLinkedList<u64, 4>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let old_len = list.len();
    let _idx = list.push(new_value);
    assert(list.len() == old_len + 1);
}

// Test 4: Push preserves uniqueness
fn test_push_preserves_unique(list: &mut StaticLinkedList<u64, 4>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let _idx = list.push(new_value);
    assert(list.unique());
}

// Test 5: Push returns node ref for the new value
fn test_push_returns_ref(list: &mut StaticLinkedList<u64, 4>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let idx = list.push(new_value);
    assert(list.get_node_ref(*new_value) == idx);
}

// Test 6: Push preserves node refs of existing values
fn test_push_preserves_existing_refs(list: &mut StaticLinkedList<u64, 4>, new_value: &u64, existing: u64)
    requires
        old(list).wf(),
        old(list).len() < 4,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
        old(list)@.contains(existing),
{
    let ghost old_ref = list.get_node_ref(existing);
    let _idx = list.push(new_value);
    assert(list.get_node_ref(existing) == old_ref);
}

// Test 7: seq_push_lemma — pushed value is contained
proof fn test_seq_push_contains() {
    seq_push_lemma::<u64>();
    let s: Seq<u64> = seq![1u64, 2u64, 3u64];
    assert(s.push(4u64).contains(4u64));
}

// Test 8: seq_push_lemma — existing values remain contained
proof fn test_seq_push_preserves_existing() {
    seq_push_lemma::<u64>();
    let s: Seq<u64> = seq![1u64, 2u64, 3u64];
    assert(s.contains(1u64));
    assert(s.push(4u64).contains(1u64));
}

// Test 9: seq_push_lemma — non-members stay non-members
proof fn test_seq_push_non_member() {
    seq_push_lemma::<u64>();
    let s: Seq<u64> = seq![1u64, 2u64, 3u64];
    assert(!s.contains(5u64));
    assert(4u64 != 5u64);
    assert(!s.push(4u64).contains(5u64));
}

// Test 10: seq_push_index_of_lemma — index preserved
proof fn test_seq_push_index_preserved() {
    seq_push_index_of_lemma::<u64>();
    seq_push_lemma::<u64>();
    let s: Seq<u64> = seq![10u64, 20u64, 30u64];
    assert(s.no_duplicates());
    assert(s.contains(10u64));
    assert(10u64 != 40u64);
    assert(s.push(40u64).index_of(10u64) == s.index_of(10u64));
}

// Test 11: Two consecutive pushes maintain well-formedness
fn test_two_pushes(list: &mut StaticLinkedList<u64, 8>, v1: &u64, v2: &u64)
    requires
        old(list).wf(),
        old(list).len() < 6,
        old(list).unique(),
        old(list)@.contains(*v1) == false,
        old(list)@.contains(*v2) == false,
        *v1 != *v2,
{
    let old_len = list.len();
    let ghost old_view = list@;

    let idx1 = list.push(v1);
    assert(list.wf());
    assert(list.len() == old_len + 1);
    assert(list.unique());
    assert(list@ == old_view.push(*v1));

    proof {
        seq_push_lemma::<u64>();
    }
    assert(!list@.contains(*v2));

    let idx2 = list.push(v2);
    assert(list.wf());
    assert(list.len() == old_len + 2);
    assert(list.unique());
}

// Test 12: Push with larger N
fn test_push_large_n(list: &mut StaticLinkedList<u64, 16>, new_value: &u64)
    requires
        old(list).wf(),
        old(list).len() < 16,
        old(list).unique(),
        old(list)@.contains(*new_value) == false,
{
    let old_len = list.len();
    let _idx = list.push(new_value);
    assert(list.wf());
    assert(list.len() == old_len + 1);
}

// Test 13: seq_push_lemma — empty sequence
proof fn test_seq_push_empty() {
    seq_push_lemma::<u64>();
    let s: Seq<u64> = Seq::empty();
    assert(s.push(42u64).contains(42u64));
}

// Test 14: seq_push_index_of_lemma — all existing indices preserved
proof fn test_seq_push_all_indices_preserved() {
    seq_push_index_of_lemma::<u64>();
    seq_push_lemma::<u64>();
    let s: Seq<u64> = seq![100u64, 200u64, 300u64];
    assert(s.no_duplicates());

    assert(s.contains(200u64));
    assert(200u64 != 400u64);
    assert(s.push(400u64).index_of(200u64) == s.index_of(200u64));

    assert(s.contains(300u64));
    assert(300u64 != 400u64);
    assert(s.push(400u64).index_of(300u64) == s.index_of(300u64));
}

} // verus!
