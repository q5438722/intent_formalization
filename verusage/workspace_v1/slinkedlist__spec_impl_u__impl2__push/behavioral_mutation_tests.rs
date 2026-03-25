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
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but mutate expected
// outputs/relations to check if incorrect behaviors are rejected.
// Each test should FAIL verification.
// ============================================================

// Test 1: push should increase length by 1, not by 2.
// Mutate: assert post length == pre length + 2.
// SHOULD FAIL
proof fn test_mutation_push_length_increases_by_two(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() < 4,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
{
    // Mutated: claim length increases by 2 instead of 1
    assert(sll_post.len() == sll.len() + 2);
}

// Test 2: push should produce self@ == old@.push(new_value), not push a different value.
// Mutate: assert post sequence equals push of a different value.
// SHOULD FAIL
proof fn test_mutation_push_wrong_value(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64, other_value: u64)
    requires
        sll.wf(),
        sll.len() == 2,
        sll.unique(),
        !sll@.contains(new_value),
        !sll@.contains(other_value),
        new_value != other_value,
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
{
    // Mutated: claim sequence has other_value instead of new_value at end
    assert(sll_post@[sll_post@.len() - 1] == other_value);
}

// Test 3: push should keep length unchanged (mutated — should be +1).
// Mutate: assert post length == pre length (no change).
// SHOULD FAIL
proof fn test_mutation_push_length_unchanged(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() == 2,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post.len() == sll.len() + 1,
{
    // Mutated: claim length is unchanged
    assert(sll_post.len() == sll.len());
}

// Test 4: After push, the last element should be the new value.
// Mutate: assert the last element equals the first element of the original list.
// SHOULD FAIL
proof fn test_mutation_push_last_element_wrong(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() == 2,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
        sll_post.len() > 0,
{
    // After push, the last element should be new_value, not sll@[0]
    // Mutated: claim last element is the first element of original
    assert(sll_post@[sll_post@.len() - 1] == sll@[0]);
}

// Test 5: After push, the first element should be preserved (not the new value).
// Mutate: assert the first element of post is the new_value.
// SHOULD FAIL
proof fn test_mutation_push_first_element_changed(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, new_value: u64)
    requires
        sll.wf(),
        sll.len() == 2,
        sll.unique(),
        !sll@.contains(new_value),
        sll_post.wf(),
        sll_post@ == sll@.push(new_value),
        sll_post.len() == sll.len() + 1,
{
    // push appends to end, so sll_post@[0] == sll@[0], not new_value.
    // new_value != sll@[0] because !sll@.contains(new_value).
    // Mutated: claim first element is the new value
    assert(sll_post@[0] == new_value);
}

}
