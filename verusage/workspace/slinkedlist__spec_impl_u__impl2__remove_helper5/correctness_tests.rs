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

    pub fn remove_helper5(&mut self, remove_index: SLLIndex, v:Ghost<T>) -> (ret: T)
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
        proof {
            seq_push_lemma::<SLLIndex>();
            seq_skip_lemma::<SLLIndex>();
            seq_skip_lemma::<T>();
            seq_skip_index_of_lemma::<T>();
        }
        let ret = self.get_value(remove_index).unwrap();
        let new_value_list_head = self.get_next(remove_index);
        self.value_list_head = new_value_list_head;
        self.set_prev(new_value_list_head, -1);
        proof {
            self.value_list@ = self.value_list@.skip(1);
            self.spec_seq@ = self.spec_seq@.skip(1);
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


// ===== CORRECTNESS TESTS (should all PASS) =====

// --- Tests for seq_push_lemma ---

// Test 1: Pushed element is always contained (parameterized)
proof fn test_push_contains_pushed(s: Seq<int>, v: int)
{
    seq_push_lemma::<int>();
    assert(s.push(v).contains(v));
}

// Test 2: Push preserves existing elements (parameterized)
proof fn test_push_preserves_existing(s: Seq<int>, existing: int, new_val: int)
    requires s.contains(existing)
{
    seq_push_lemma::<int>();
    assert(s.push(new_val).contains(existing));
}

// Test 3: Push doesn't add unrelated elements (parameterized)
proof fn test_push_no_spurious(s: Seq<int>, v: int, x: int)
    requires !s.contains(x), v != x
{
    seq_push_lemma::<int>();
    assert(!s.push(v).contains(x));
}

// Test 4: Push on empty seq - concrete
proof fn test_push_empty_concrete()
{
    seq_push_lemma::<int>();
    let s = Seq::<int>::empty();
    assert(s.push(42).contains(42));
}

// --- Tests for seq_skip_lemma ---

// Test 5: First element of non-empty seq is contained (parameterized)
proof fn test_first_element_contained(s: Seq<int>)
    requires s.len() > 0
{
    seq_skip_lemma::<int>();
    assert(s.contains(s[0]));
}

// Test 6: Skip preserves containment of non-first elements (parameterized)
proof fn test_skip_preserves_non_first(s: Seq<int>, v: int)
    requires s.len() > 0, s[0] != v, s.no_duplicates(), s.contains(v)
{
    seq_skip_lemma::<int>();
    assert(s.skip(1).contains(v));
}

// Test 7: First element not in skip(1) when no_duplicates (parameterized)
proof fn test_skip_removes_first(s: Seq<int>)
    requires s.len() > 0, s.no_duplicates()
{
    seq_skip_lemma::<int>();
    assert(!s.skip(1).contains(s[0]));
}

// Test 8: skip(1) equals remove_value of first when no_duplicates (parameterized)
proof fn test_skip_is_remove_value(s: Seq<int>, v: int)
    requires s.len() > 0, s[0] == v, s.no_duplicates()
{
    seq_skip_lemma::<int>();
    assert(s.skip(1) =~= s.remove_value(v));
}

// Test 9: skip(1) indexing (parameterized)
proof fn test_skip_indexing(s: Seq<int>, i: int)
    requires 0 <= i < s.len() - 1
{
    seq_skip_lemma::<int>();
    assert(s.skip(1)[i] == s[i + 1]);
}

// --- Tests for seq_skip_index_of_lemma ---

// Test 10: index_of after skip (parameterized)
proof fn test_skip_index_of(s: Seq<int>, v: int)
    requires
        s.len() != 0,
        s.no_duplicates(),
        s.contains(v),
        s[0] != v,
{
    seq_skip_index_of_lemma::<int>();
    assert(s.skip(1).index_of(v) == s.index_of(v) - 1);
}

// --- Tests for open spec functions ---

// Test 11: spec_len is consistent with view length
proof fn test_spec_len_equals_view_len<T, const N: usize>(sll: StaticLinkedList<T, N>) {
    assert(sll.spec_len() == sll@.len() as usize);
}

// Test 12: view returns spec_seq
proof fn test_view_returns_spec_seq<T, const N: usize>(sll: StaticLinkedList<T, N>) {
    assert(sll@ == sll.spec_seq@);
}

// --- Tests for postcondition consistency ---

// Test 13: ret == v implies remove_value(ret) == remove_value(v)
proof fn test_ret_equals_v_consistency(s: Seq<int>, ret: int, v: int)
    requires ret == v
{
    assert(s.remove_value(ret) =~= s.remove_value(v));
}

// Test 14: Combined postcondition test - if new_view =~= old_view.remove_value(ret) and
// old_view contains ret, then new_view length equals old_view remove_value length
proof fn test_postcondition_combined(old_view: Seq<int>, new_view: Seq<int>, ret: int)
    requires
        new_view =~= old_view.remove_value(ret),
        old_view.contains(ret),
{
    assert(new_view.len() == old_view.remove_value(ret).len());
}

// Test 15: Combining push and skip lemmas (as used in remove_helper5 body)
proof fn test_push_and_skip_combined(s: Seq<int>, v: int)
    requires
        s.len() > 0,
        s[0] == v,
        s.no_duplicates(),
{
    seq_push_lemma::<int>();
    seq_skip_lemma::<int>();
    // skip(1) is remove_value(v)
    assert(s.skip(1) =~= s.remove_value(v));
    // The skipped sequence doesn't contain v (since no_duplicates)
    assert(!s.skip(1).contains(v));
}

}
