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


impl<T: Copy, const N: usize> StaticLinkedList<T, N> {

    #[verifier::rlimit(200)]
    pub fn remove_helper7(&mut self, remove_index: SLLIndex, v:Ghost<T>) -> (ret: T)
        requires
            old(self).wf(),
            old(self)@.contains(v@),
            old(self).get_node_ref(v@) == remove_index,
            old(self).value_list_len != 1,
            old(self).free_list_len != 0 && old(self).value_list_tail != remove_index && old(
                self,
            ).value_list_head != remove_index && old(self).value_list_len != 1,
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
        proof {
            seq_push_lemma::<SLLIndex>();
            seq_skip_lemma::<SLLIndex>();
            seq_remove_lemma::<SLLIndex>();
            seq_remove_lemma_2::<SLLIndex>();
            seq_remove_lemma::<T>();
        }
        let ret = self.get_value(remove_index).unwrap();
        let prev = self.get_prev(remove_index);
        let next = self.get_next(remove_index);
        self.set_next(prev, next);
        self.set_prev(next, prev);

        let ghost_index = Ghost(self.value_list@.index_of(remove_index));
        assert(0 <= ghost_index@ < self.value_list@.len());
        assert(self.value_list@[ghost_index@] == remove_index);

        proof {
            self.value_list@ = self.value_list@.subrange(0, ghost_index@).add(
                self.value_list@.subrange(ghost_index@ + 1, self.value_list_len as int),
            );
            self.spec_seq@ = self.spec_seq@.subrange(0, ghost_index@).add(
                self.spec_seq@.subrange(ghost_index@ + 1, self.value_list_len as int),
            );
        }
        self.value_list_len = self.value_list_len - 1;

        let old_free_list_tail = self.free_list_tail;
        self.set_next(old_free_list_tail, remove_index);
        self.set_next(remove_index, -1);
        self.set_prev(remove_index, old_free_list_tail);
        self.free_list_tail = remove_index;
        self.free_list_len = self.free_list_len + 1;
        proof {
            self.free_list@ = self.free_list@.push(remove_index);
        }

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

    #[verifier::external_body]
    #[verifier(external_body)]
    pub fn get_next(&self, index: SLLIndex) -> (next: SLLIndex)
        requires
            0 <= index < N,
            self.array_wf(),
        ensures
            next == self.arr_seq@[index as int].next,
    {
        unimplemented!()
    }

    #[verifier::external_body]
    #[verifier(external_body)]
    pub fn get_prev(&self, index: SLLIndex) -> (prev: SLLIndex)
        requires
            0 <= index < N,
            self.array_wf(),
        ensures
            prev == self.arr_seq@[index as int].prev,
    {
        unimplemented!()
    }

}


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

    #[verifier::external_body]
pub proof fn seq_remove_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] != v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ).contains(v),
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ).contains(v) == false,
        forall|s: Seq<A>, i: int, j: int|
            #![trigger s.subrange(0,i), s[j]]
            0 <= i < s.len() && 0 <= j < i ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j] == s[j],
        forall|s: Seq<A>, i: int, j: int|
            #![trigger s.subrange(0,i), s[j+1]]
            0 <= i < s.len() && i <= j < s.len() - 1 ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j]
                == s[j + 1],
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.remove_value(v), s.subrange(0,i)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ) == s.remove_value(v),
    {
        unimplemented!()
    }

    #[verifier::external_body]
pub proof fn seq_remove_lemma_2<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A|
            x != v && s.no_duplicates() ==> s.remove_value(x).contains(v) == s.contains(v),
        forall|s: Seq<A>, v: A|
            #![auto]
            s.no_duplicates() ==> s.remove_value(v).contains(v) == false,
    {
        unimplemented!()
    }


// ===================== CORRECTNESS TESTS =====================
// These tests should all PASS (verify successfully).

// Test 1: wf() is preserved after removal
fn test_wf_preserved(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper7(remove_index, v);
    assert(sll.wf());
}

// Test 2: unique() is preserved after removal
fn test_unique_preserved(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper7(remove_index, v);
    assert(sll.unique());
}

// Test 3: return value equals the ghost value v
fn test_ret_equals_v(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper7(remove_index, v);
    proof {
        assert(ret == v@);
    }
}

// Test 4: view equals old view with removed value
fn test_view_remove(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ghost old_view = sll@;
    let ret = sll.remove_helper7(remove_index, v);
    proof {
        assert(sll@ =~= old_view.remove_value(ret));
    }
}

// Test 5: length decreases by exactly 1
fn test_len_decreases(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let old_len = sll.len();
    let ret = sll.remove_helper7(remove_index, v);
    let new_len = sll.len();
    proof {
        assert(new_len as int == old_len as int - 1);
    }
}

// Test 6: node references preserved for elements still in the list
fn test_node_refs_preserved(
    sll: &mut StaticLinkedList<u64, 10>,
    remove_index: SLLIndex,
    v: Ghost<u64>,
    w: Ghost<u64>,
)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
        old(sll)@.contains(w@),
        w@ != v@,
{
    let ghost old_node_ref = sll.get_node_ref(w@);
    let ret = sll.remove_helper7(remove_index, v);
    proof {
        seq_remove_lemma_2::<u64>();
        assert(sll@.contains(w@));
        assert(sll.get_node_ref(w@) == old_node_ref);
    }
}

// Test 7: all postconditions together
fn test_all_postconditions(sll: &mut StaticLinkedList<u64, 10>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ghost old_view = sll@;
    let old_len = sll.len();
    let ret = sll.remove_helper7(remove_index, v);
    let new_len = sll.len();
    assert(sll.wf());
    assert(sll.unique());
    proof {
        assert(ret == v@);
        assert(sll@ =~= old_view.remove_value(ret));
        assert(new_len as int == old_len as int - 1);
    }
}

// Test 8: with minimum valid N=3
fn test_min_n(sll: &mut StaticLinkedList<u64, 3>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper7(remove_index, v);
    assert(sll.wf());
    proof {
        assert(ret == v@);
    }
}

// Test 9: with large N=1000
fn test_large_n(sll: &mut StaticLinkedList<u64, 1000>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper7(remove_index, v);
    assert(sll.wf());
    proof {
        assert(ret == v@);
    }
}

// Test 10: with a different type (i32)
fn test_generic_type(sll: &mut StaticLinkedList<i32, 10>, remove_index: SLLIndex, v: Ghost<i32>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len != 0,
        old(sll).value_list_tail != remove_index,
        old(sll).value_list_head != remove_index,
{
    let ghost old_view = sll@;
    let ret = sll.remove_helper7(remove_index, v);
    assert(sll.wf());
    assert(sll.unique());
    proof {
        assert(ret == v@);
        assert(sll@ =~= old_view.remove_value(ret));
    }
}

}
