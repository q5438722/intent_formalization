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
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// ============================================================

// Test 1: The spec does not constrain the actual values stored in the list.
// The returned value could be any u64, including 0.
// Asserting ret > 0 is unwarranted.
// SHOULD FAIL
proof fn test_logical_value_constrained(sll: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll@.len() == 1,
        ret == sll@[0],
{
    // The spec says nothing about the magnitude of ret
    // It could be 0 or any u64; asserting it's positive is unwarranted
    assert(ret > 0);
}

// Test 2: After removing 1 element, free_list_len should grow by exactly 1.
// Assert it grows by 2 instead — this should fail.
// SHOULD FAIL
proof fn test_logical_free_list_grows_by_two(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>)
    requires
        sll.free_list_len + sll.value_list_len == 4,
        sll.value_list_len == 1,
        sll_post.free_list_len + sll_post.value_list_len == 4,
        sll_post.value_list_len == 0,
{
    // sll.free_list_len == 3, sll_post.free_list_len == 4
    // Difference is 1, not 2
    assert(sll_post.free_list_len == sll.free_list_len + 2);
}

// Test 3: The postcondition preserves get_node_ref only for values still
// in the list. Assert it is also preserved for the REMOVED value.
// Since ret was removed, this guarantee does not apply.
// SHOULD FAIL
proof fn test_logical_node_ref_preserved_for_removed(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll.wf(),
        sll@.len() == 1,
        ret == sll@[0],
        sll_post.wf(),
        sll_post.len() == sll.len() - 1,
        sll_post@ =~= sll@.remove_value(ret),
        sll@.no_duplicates(),
{
    // get_node_ref is only preserved for self@.contains(v)
    // ret is no longer in sll_post@, so this should NOT be provable
    assert(sll.get_node_ref(ret) == sll_post.get_node_ref(ret));
}

// Test 4: wf() ensures size == N for both pre and post states.
// But since wf() is closed, the verifier cannot derive this.
// Assert that size changes — this should fail (not provable).
// SHOULD FAIL
proof fn test_logical_array_size_changes(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>)
    requires
        sll.wf(),
        sll_post.wf(),
{
    // wf() includes array_wf() which includes size == N
    // Both should have size == 4, so they should be equal
    // But wf() is closed — verifier can't derive this
    assert(sll_post.size != sll.size);
}

// Test 5: The spec does not determine which specific array slot a value
// occupies. get_node_ref returns a value_list index, but the particular
// slot is an implementation detail. Asserting a specific index should fail.
// SHOULD FAIL
proof fn test_logical_index_specific_value(sll: StaticLinkedList<u64, 4>, remove_index: SLLIndex)
    requires
        sll.wf(),
        sll@.len() == 1,
        sll@.contains(sll@[0]),
        remove_index == sll.get_node_ref(sll@[0]),
{
    // get_node_ref returns value_list@[index_of(v)], which is some slot in [0, N).
    // The spec does not pin it to slot 0 — the allocation is implementation-defined.
    assert(remove_index == 0);
}

}
