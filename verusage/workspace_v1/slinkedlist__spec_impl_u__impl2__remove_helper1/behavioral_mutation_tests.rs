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
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but mutate expected
// outputs/relations to check if incorrect behaviors are rejected.
// ============================================================

// Test 1: remove_helper1 should return ret == v (the removed value).
// Mutate: assert ret is a specific wrong value (99) instead of the actual value (42).
// SHOULD FAIL
proof fn test_mutation_wrong_return_value(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll@.len() == 1,
        sll@[0] == 42u64,
        ret == sll@[0],
        sll_post@ =~= sll@.remove_value(ret),
        sll@.no_duplicates(),
{
    // ret == sll@[0] == 42, but mutated claim says ret == 99
    assert(ret == 99u64);
}

// Test 2: remove_helper1 should reduce length by 1.
// Mutate: assert post length equals pre length (no decrease).
// SHOULD FAIL
proof fn test_mutation_length_unchanged(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>)
    requires
        sll@.len() == 1,
        sll_post.len() == sll.len() - 1,
{
    // sll.len() == 1, sll_post.len() == 0
    // Mutated: claim length is unchanged
    assert(sll_post.len() == sll.len());
}

// Test 3: remove_helper1 should produce self@ =~= old@.remove_value(ret).
// Mutate: assert the sequence is unchanged (nothing removed).
// SHOULD FAIL
proof fn test_mutation_seq_unchanged(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll@.len() == 1,
        sll@.no_duplicates(),
        ret == sll@[0],
        sll_post@ =~= sll@.remove_value(ret),
{
    seq_skip_lemma::<u64>();
    // sll_post@ =~= sll@.remove_value(ret) =~= sll@.skip(1), which has len 0
    // sll@ has len 1. So sll_post@ != sll@
    // Mutated: claim sequence is unchanged
    assert(sll_post@ =~= sll@);
}

// Test 4: After removing the only element, post sequence has len 0.
// Mutate: assert post sequence still has 1 element.
// SHOULD FAIL
proof fn test_mutation_post_has_element(sll: StaticLinkedList<u64, 4>, sll_post: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll@.len() == 1,
        sll@.no_duplicates(),
        ret == sll@[0],
        sll_post@ =~= sll@.remove_value(ret),
{
    seq_skip_lemma::<u64>();
    // After removing the only element, post should be empty
    // Mutated: assert it still has 1 element
    assert(sll_post@.len() == 1);
}

// Test 5: remove_helper1 removes the contained value.
// Mutate: assert ret is different from the pre-state's element.
// SHOULD FAIL
proof fn test_mutation_ret_differs_from_element(sll: StaticLinkedList<u64, 4>, ret: u64)
    requires
        sll@.len() == 1,
        ret == sll@[0],
{
    // ret == sll@[0] by the postcondition (ret == v@)
    // Mutated: claim ret differs from the stored element
    assert(ret != sll@[0]);
}

}
