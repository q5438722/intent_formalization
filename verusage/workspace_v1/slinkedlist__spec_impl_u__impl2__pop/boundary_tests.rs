use vstd::prelude::*;

fn main() {}

verus!{

pub type SLLIndex = i32;

// File: slinkedlist/node.rs
#[derive(Debug)]
pub struct Node<T> {
    pub value: Option<T>,
    pub next: SLLIndex,
    pub prev: SLLIndex,
}


// File: slinkedlist/spec_impl_u.rs
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

    pub fn pop(&mut self) -> (ret: (T, SLLIndex))
        requires
            old(self).wf(),
            old(self).len() > 0,
            old(self).unique(),
            N > 2,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            self@ == old(self)@.skip(1),
            ret.0 == old(self)@[0],
            ret.1 == old(self).get_node_ref(ret.0),
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
        if self.free_list_len == 0 {
            let ret_index = self.value_list_head;
            let ret = self.get_value(ret_index).unwrap();

            let new_value_list_head = self.get_next(ret_index);
            self.value_list_head = new_value_list_head;
            self.set_prev(new_value_list_head, -1);
            proof {
                self.value_list@ = self.value_list@.skip(1);
                self.spec_seq@ = self.spec_seq@.skip(1);
            }
            self.value_list_len = self.value_list_len - 1;

            self.free_list_head = ret_index;
            self.free_list_tail = ret_index;
            self.set_prev(ret_index, -1);
            self.set_next(ret_index, -1);
            proof {
                self.free_list@ = self.free_list@.push(ret_index);
            }
            self.free_list_len = self.free_list_len + 1;

            assert(self.wf());
            return (ret, ret_index);
        } else if self.value_list_len == 1 {
            let ret_index = self.value_list_head;
            let ret = self.get_value(ret_index).unwrap();

            let old_free_list_tail = self.free_list_tail;
            self.set_next(old_free_list_tail, ret_index);
            self.set_prev(ret_index, old_free_list_tail);
            self.free_list_tail = ret_index;
            self.free_list_len = self.free_list_len + 1;
            proof {
                self.free_list@ = self.free_list@.push(ret_index);
            }

            self.value_list_head = -1;
            self.value_list_tail = -1;
            proof {
                self.value_list@ = self.value_list@.skip(1);
                self.spec_seq@ = self.spec_seq@.skip(1);
            }
            self.value_list_len = self.value_list_len - 1;

            assert(self.wf());

            return (ret, ret_index);
        } else {
            let ret_index = self.value_list_head;
            let ret = self.get_value(ret_index).unwrap();

            let new_value_list_head = self.get_next(ret_index);
            self.value_list_head = new_value_list_head;
            self.set_prev(new_value_list_head, -1);
            proof {
                self.value_list@ = self.value_list@.skip(1);
                self.spec_seq@ = self.spec_seq@.skip(1);
            }
            self.value_list_len = self.value_list_len - 1;

            let old_free_list_tail = self.free_list_tail;
            self.set_next(ret_index, -1);
            self.set_next(old_free_list_tail, ret_index);
            self.set_prev(ret_index, old_free_list_tail);
            self.free_list_tail = ret_index;
            self.free_list_len = self.free_list_len + 1;
            proof {
                self.free_list@ = self.free_list@.push(ret_index);
            }
            assert(self.wf());
            return (ret, ret_index);
        }
    }

}


// File: slinkedlist/impl_t.rs
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


// File: lemma/lemma_u.rs
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


// ============================================================
// BOUNDARY TESTS
// These tests check that invalid inputs and edge-case
// violations are properly rejected by the specification.
// All requires must be satisfiable; assertions must be false.
// ============================================================

// Test 1: unique() should reject a sequence with duplicate elements.
// unique() requires all elements at distinct indices to be different.
// A sequence [42, 99, 42] has duplicates at index 0 and 2.
// SHOULD FAIL
proof fn test_boundary_unique_with_duplicates(sll: StaticLinkedList<u64, 4>)
    requires
        sll@.len() == 3,
        sll@[0] == 42,
        sll@[1] == 99,
        sll@[2] == 42,
        sll.value_list_len == 3,
{
    // unique() should be false because sll@[0] == sll@[2]
    assert(sll.unique());
}

// Test 2: spec_len should equal view().len(). Asserting a mismatch should fail.
// spec_len() is defined as self@.len() as usize, so if view has 3 elements,
// spec_len must be 3.
// SHOULD FAIL
proof fn test_boundary_spec_len_mismatch(sll: StaticLinkedList<u64, 4>)
    requires
        sll@.len() == 3,
{
    // spec_len() == self@.len() as usize == 3, so != 3 is false
    assert(sll.spec_len() != 3);
}

// Test 3: After pop from a list with len >= 2, the result cannot be empty.
// pop ensures self.len() == old(self).len() - 1. With old len >= 2, post len >= 1.
// SHOULD FAIL
proof fn test_boundary_pop_result_not_empty(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>)
    requires
        sll.wf(),
        sll.len() >= 2,
        sll.unique(),
        sll_post.wf(),
        sll_post.len() == sll.len() - 1,
{
    // sll.len() >= 2 implies sll_post.len() >= 1
    // Asserting result is empty should fail
    assert(sll_post.len() == 0);
}

// Test 4: After pop, the post-state should maintain uniqueness (via wf).
// Asserting that two distinct elements in the post-state are equal should fail.
// SHOULD FAIL
proof fn test_boundary_pop_non_unique_result(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, ret: (u64, SLLIndex))
    requires
        sll.wf(),
        sll.len() > 0,
        sll.unique(),
        sll_post.wf(),
        sll_post@ == sll@.skip(1),
        sll_post.len() == sll.len() - 1,
        sll_post.len() >= 2,
{
    // wf includes unique, so distinct indices must differ
    assert(sll_post@[0] == sll_post@[1]);
}

// Test 5: After pop from len == 1, the result must have len == 0.
// Asserting len > 0 after popping the only element should fail.
// SHOULD FAIL
proof fn test_boundary_pop_single_element_not_empty(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>)
    requires
        sll.wf(),
        sll.len() == 1,
        sll.unique(),
        sll_post.wf(),
        sll_post.len() == sll.len() - 1,
{
    // sll.len() == 1 means sll_post.len() == 0
    // Asserting post still has elements should fail
    assert(sll_post.len() > 0);
}

}
