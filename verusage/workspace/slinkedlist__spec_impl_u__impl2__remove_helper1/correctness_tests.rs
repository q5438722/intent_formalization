use vstd::prelude::*;

fn main() {}

verus! {

// === Definitions from source file ===

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

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

    pub fn remove_helper1(&mut self, remove_index: SLLIndex, v: Ghost<T>) -> (ret: T)
        requires
            old(self).wf(),
            old(self)@.contains(v@),
            old(self).get_node_ref(v@) == remove_index,
            old(self).value_list_len == 1,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            ret == v@,
            self.unique(),
            self@ =~= old(self)@.remove_value(ret),
            forall|v:T|
                #![auto]
                self@.contains(v) ==>
                    old(self).get_node_ref(v) ==
                        self.get_node_ref(v),
    {
        proof {
            seq_push_lemma::<SLLIndex>();
            seq_skip_lemma::<SLLIndex>();
            seq_skip_lemma::<T>();
            seq_skip_index_of_lemma::<T>();
        }
        let ret = self.get_value(remove_index).unwrap();
        let old_free_list_tail = self.free_list_tail;
        self.set_next(old_free_list_tail, remove_index);
        self.set_prev(remove_index, old_free_list_tail);
        self.free_list_tail = remove_index;
        self.free_list_len = self.free_list_len + 1;
        proof {
            self.free_list@ = self.free_list@.push(remove_index);
        }

        self.value_list_head = -1;
        self.value_list_tail = -1;
        proof {
            self.value_list@ = self.value_list@.skip(1);
            self.spec_seq@ = self.spec_seq@.skip(1);
        }
        self.value_list_len = self.value_list_len - 1;

        assert(self.wf());
        return ret;
    }

}

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

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
    #[verifier(external_body)]
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
    #[verifier(external_body)]
    pub fn get_value(&self, index: SLLIndex) -> (ret: Option<T>)
        requires
            0 <= index < N,
            self.array_wf(),
        ensures
            ret == self.arr_seq@[index as int].value,
    {
        unimplemented!()
    }

}


// Proof lemmas
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
pub proof fn seq_skip_index_of_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A,|
            #![auto]
            s.len() != 0 && s.no_duplicates() && s.contains(v) && s[0] != v
            ==>
            s.skip(1).index_of(v) == s.index_of(v) - 1,
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn seq_skip_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A|
            s.len() > 0 && s[0] != v && s.no_duplicates() ==> (s.skip(1).contains(v) == s.contains(v)),
        forall|s: Seq<A>| #![trigger s[0]] s.len() > 0 ==> s.contains(s[0]),
        forall|s: Seq<A>| #![trigger s[0]] s.len() > 0 && s.no_duplicates() ==> !s.skip(1).contains(s[0]),
        forall|s: Seq<A>, v: A| s.len() > 0 && s[0] == v && s.no_duplicates() ==> s.skip(1) =~= s.remove_value(v),
        forall|s: Seq<A>, i: int| 0 <= i < s.len() - 1 ==> s.skip(1)[i] == s[i + 1],
{
    unimplemented!()
}


// ============ CORRECTNESS TESTS (should all PASS) ============

// --- Tests for seq_push_lemma ---

// Test: pushing an element makes it contained
proof fn test_push_contains_pushed_element()
{
    seq_push_lemma::<int>();
    let s = Seq::<int>::empty();
    assert(s.push(42int).contains(42int));
}

// Test: pushing preserves existing elements
proof fn test_push_preserves_existing()
{
    seq_push_lemma::<int>();
    let s = seq![1int, 2, 3];
    assert(s[0] == 1int);
    assert(s.push(4int).contains(4int));
    assert(s.push(4int).contains(1int));
}

// Test: pushing does not add unrelated elements
proof fn test_push_no_unrelated()
{
    seq_push_lemma::<int>();
    let s = Seq::<int>::empty();
    assert(!s.push(42int).contains(10int));
}

// Test: seq_push_lemma with parameterized input
proof fn test_push_param(v: int, x: int)
    requires v != x
{
    seq_push_lemma::<int>();
    let s = Seq::<int>::empty();
    assert(s.push(v).contains(v));
    assert(!s.push(v).contains(x));
}

// --- Tests for seq_skip_lemma ---

// Test: skip(1) indexing matches original shifted by 1
proof fn test_skip_indexing()
{
    seq_skip_lemma::<int>();
    let s = seq![10int, 20, 30, 40];
    assert(s.skip(1)[0] == s[1]);
    assert(s.skip(1)[1] == s[2]);
    assert(s.skip(1)[2] == s[3]);
}

// Test: first element is contained in a non-empty sequence
proof fn test_first_element_contained()
{
    seq_skip_lemma::<int>();
    let s = seq![5int, 10, 15];
    assert(s.contains(s[0]));
}

// Test: skip(1) does not contain first element when no duplicates
proof fn test_skip_removes_first()
{
    seq_skip_lemma::<int>();
    let s = seq![5int, 10, 15];
    assert(s.no_duplicates());
    assert(!s.skip(1).contains(s[0]));
}

// Test: skip(1) == remove_value(s[0]) when no duplicates
proof fn test_skip_is_remove_value()
{
    seq_skip_lemma::<int>();
    let s = seq![5int, 10, 15];
    assert(s.no_duplicates());
    assert(s.skip(1) =~= s.remove_value(s[0]));
}

// Test: skip preserves containment of non-first elements
proof fn test_skip_preserves_non_first()
{
    seq_skip_lemma::<int>();
    let s = seq![1int, 2, 3];
    assert(s.no_duplicates());
    assert(s[0] != 2int);
    assert(s.skip(1).contains(2int) == s.contains(2int));
}

// Test: parameterized - skip(1) indexing
proof fn test_skip_indexing_param(s: Seq<int>)
    requires s.len() >= 3
{
    seq_skip_lemma::<int>();
    assert(s.skip(1)[0] == s[1]);
    assert(s.skip(1)[1] == s[2]);
}

// --- Tests for seq_skip_index_of_lemma ---

// Test: index_of decreases by 1 after skip(1)
proof fn test_skip_index_of()
{
    seq_skip_index_of_lemma::<int>();
    seq_skip_lemma::<int>();
    let s = seq![10int, 20, 30];
    assert(s.no_duplicates());
    assert(s.contains(20int));
    assert(s[0] != 20int);
    assert(s.skip(1).index_of(20int) == s.index_of(20int) - 1);
}

// Test: index_of for last element
proof fn test_skip_index_of_last()
{
    seq_skip_index_of_lemma::<int>();
    seq_skip_lemma::<int>();
    let s = seq![10int, 20, 30];
    assert(s.no_duplicates());
    assert(s.contains(30int));
    assert(s[0] != 30int);
    assert(s.skip(1).index_of(30int) == s.index_of(30int) - 1);
}

// --- Tests for remove_helper1 spec (assumption-based) ---

// Test: postconditions of remove_helper1 are mutually consistent
proof fn test_remove_spec_consistency<T: Copy, const N: usize>(
    old_sll: StaticLinkedList<T, N>,
    new_sll: StaticLinkedList<T, N>,
    remove_index: SLLIndex,
    v: T,
    ret: T,
)
    requires
        old_sll.wf(),
        old_sll@.contains(v),
        old_sll.get_node_ref(v) == remove_index,
        old_sll.value_list_len == 1,
        new_sll.wf(),
        new_sll.spec_len() == old_sll.spec_len() - 1,
        ret == v,
        new_sll.unique(),
        new_sll@ =~= old_sll@.remove_value(ret),
{
    assert(ret == v);
    assert(new_sll.wf());
    assert(new_sll.unique());
    assert(new_sll@ =~= old_sll@.remove_value(v));
}

// Test: postconditions preserve node refs for remaining elements
proof fn test_remove_preserves_node_refs<T: Copy, const N: usize>(
    old_sll: StaticLinkedList<T, N>,
    new_sll: StaticLinkedList<T, N>,
    remove_index: SLLIndex,
    v: T,
    ret: T,
    w: T,
)
    requires
        old_sll.wf(),
        old_sll@.contains(v),
        old_sll.get_node_ref(v) == remove_index,
        old_sll.value_list_len == 1,
        new_sll.wf(),
        new_sll.spec_len() == old_sll.spec_len() - 1,
        ret == v,
        new_sll.unique(),
        new_sll@ =~= old_sll@.remove_value(ret),
        forall|u: T| #[trigger] new_sll@.contains(u) ==>
            old_sll.get_node_ref(u) == new_sll.get_node_ref(u),
        new_sll@.contains(w),
{
    assert(old_sll.get_node_ref(w) == new_sll.get_node_ref(w));
}

// Test: length relationship is consistent
proof fn test_remove_length_relation<T: Copy, const N: usize>(
    old_sll: StaticLinkedList<T, N>,
    new_sll: StaticLinkedList<T, N>,
    remove_index: SLLIndex,
    v: T,
    ret: T,
)
    requires
        old_sll.wf(),
        old_sll@.contains(v),
        old_sll.get_node_ref(v) == remove_index,
        old_sll.value_list_len == 1,
        new_sll.wf(),
        new_sll.spec_len() == old_sll.spec_len() - 1,
        ret == v,
        new_sll@ =~= old_sll@.remove_value(ret),
{
    assert(new_sll.spec_len() + 1 == old_sll.spec_len());
}

// Test: multiple pushes preserve containment
proof fn test_push_chain()
{
    seq_push_lemma::<int>();
    let s = Seq::<int>::empty();
    let s1 = s.push(1int);
    assert(s1.contains(1int));
    let s2 = s1.push(2int);
    assert(s2.contains(1int));
    assert(s2.contains(2int));
}

// Test: skip(1) on length-1 sequence
proof fn test_skip_singleton()
{
    seq_skip_lemma::<int>();
    let s = seq![42int];
    assert(s.len() > 0);
    assert(s[0] == 42int);
    assert(s.no_duplicates());
    assert(s.skip(1) =~= s.remove_value(42int));
}

} // verus!
