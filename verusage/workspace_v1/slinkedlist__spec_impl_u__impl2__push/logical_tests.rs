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


// ============================================================
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// Each test should FAIL verification.
// ============================================================

// Test 1: The spec does not constrain the returned index to a specific value.
// The returned free_node_index depends on internal free list state.
// Asserting it is always 0 is unwarranted.
// SHOULD FAIL
proof fn test_logical_push_return_always_zero(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64, ret: SLLIndex)
    requires
        sll.wf(),
        sll.len() < 4,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
        ret == sll_post.get_node_ref(new_value),
{
    // The spec does not guarantee the index is always 0
    assert(ret == 0);
}

// Test 2: The spec does not guarantee that the returned index is negative.
// value_list indices are bounded to [0, N) by wf(), so ret >= 0.
// Asserting ret < 0 should fail.
// SHOULD FAIL
proof fn test_logical_push_return_negative(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64, ret: SLLIndex)
    requires
        sll.wf(),
        sll.len() < 4,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
        sll_post.unique(),
        ret == sll_post.get_node_ref(new_value),
{
    // value_list indices are in [0, N), so ret cannot be negative
    assert(ret < 0);
}

// Test 3: The spec does not constrain the stored values.
// Asserting that a pushed value must be > 0 is unwarranted.
// SHOULD FAIL
proof fn test_logical_pushed_value_must_be_positive(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() == 1,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
{
    // The spec says nothing about new_value > 0; it could be 0
    assert(new_value > 0);
}

// Test 4: The spec does not guarantee that the free_list_len decreases by 2.
// Since push consumes one free node, free_list_len should decrease by exactly 1.
// Asserting it decreases by 2 should fail.
// SHOULD FAIL
proof fn test_logical_free_list_shrinks_by_two(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() < 4,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post.len() == sll.len() + 1,
        sll.free_list_len + sll.value_list_len == 4,
        sll_post.free_list_len + sll_post.value_list_len == 4,
{
    // From wf: free + value = N. If value increases by 1, free decreases by 1.
    // Assert free decreases by 2 — should fail.
    assert(sll_post.free_list_len == sll.free_list_len - 2);
}

// Test 5: The spec does not guarantee that the array size field changes after push.
// wf() ensures size == N and array_wf() preserves this.
// Asserting size changes should fail.
// SHOULD FAIL
proof fn test_logical_array_size_changes_after_push(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() < 4,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post.len() == sll.len() + 1,
{
    // wf() ensures array_wf() which ensures size == N.
    // Both pre and post are wf, so both have size == 4.
    // Asserting size changes should fail.
    assert(sll_post.size != sll.size);
}

}
