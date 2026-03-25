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
    pub open spec fn spec_len(&self) -> usize { self@.len() as usize }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (l: usize)
        ensures l == self.value_list_len, self.wf() ==> l == self.len(), self.wf() ==> l == self@.len(),
    { unimplemented!() }
    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int| #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self.len() && 0 <= j < self.len() && i != j ==> self.spec_seq@[i] != self.spec_seq@[j]
    }
    pub open spec fn view(&self) -> Seq<T> { self.spec_seq@ }
    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex
        recommends self.wf(), self@.contains(v),
    { self.value_list@[self@.index_of(v)] }
    pub closed spec fn prev_free_node_of(&self, i: nat) -> int
        recommends i < self.free_list@.len(),
    { if i == 0 { -1 } else { self.free_list@[i - 1int] as int } }
    pub closed spec fn next_free_node_of(&self, i: nat) -> int
        recommends i < self.free_list@.len(),
    { if i + 1 == self.free_list@.len() { -1 } else { self.free_list@[i + 1int] as int } }
    pub closed spec fn wf_free_node_head(&self) -> bool {
        if self.free_list@.len() == 0 { self.free_list_head == -1 } else { self.free_list_head == self.free_list@[0] }
    }
    pub closed spec fn wf_free_node_tail(&self) -> bool {
        if self.free_list@.len() == 0 { self.free_list_tail == -1 }
        else { self.free_list_tail == self.free_list@[self.free_list@.len() - 1] }
    }
    pub closed spec fn free_list_wf(&self) -> bool {
        &&& forall|i: nat| #![trigger self.arr_seq@[self.free_list@[i as int] as int].next] #![trigger self.next_free_node_of(i)]
            0 <= i < self.free_list@.len() ==> self.arr_seq@[self.free_list@[i as int] as int].next == self.next_free_node_of(i)
        &&& forall|i: nat| #![trigger self.arr_seq@[self.free_list@[i as int] as int].prev] #![trigger self.prev_free_node_of(i)]
            0 <= i < self.free_list@.len() ==> self.arr_seq@[self.free_list@[i as int] as int].prev == self.prev_free_node_of(i)
        &&& forall|i: nat| #![trigger self.free_list@[i as int]]
            0 <= i < self.free_list@.len() ==> 0 <= self.free_list@[i as int] < N
        &&& forall|i: int, j: int| #![trigger self.free_list@[i], self.free_list@[j]]
            0 <= i < self.free_list_len && 0 <= j < self.free_list_len && i != j ==> self.free_list@[i] != self.free_list@[j]
        &&& self.wf_free_node_head()
        &&& self.wf_free_node_tail()
        &&& self.free_list_len == self.free_list@.len()
    }
    pub closed spec fn prev_value_node_of(&self, i: int) -> int
        recommends 0 <= i < self.value_list@.len(),
    { if i == 0 { -1 } else { self.value_list@[i - 1int] as int } }
    pub closed spec fn next_value_node_of(&self, i: int) -> int
        recommends 0 <= i < self.value_list@.len(),
    { if i + 1 == self.value_list@.len() { -1 } else { self.value_list@[i + 1int] as int } }
    pub closed spec fn wf_value_node_head(&self) -> bool {
        if self.value_list@.len() == 0 { self.value_list_head == -1 } else { self.value_list_head == self.value_list@[0] }
    }
    pub closed spec fn wf_value_node_tail(&self) -> bool {
        if self.value_list@.len() == 0 { self.value_list_tail == -1 }
        else { self.value_list_tail == self.value_list@[self.value_list@.len() - 1] }
    }
    pub closed spec fn value_list_wf(&self) -> bool {
        &&& forall|i: int| #![trigger self.arr_seq@[self.value_list@[i as int] as int].next] #![trigger self.next_value_node_of(i)]
            0 <= i < self.value_list@.len() ==> self.arr_seq@[self.value_list@[i as int] as int].next == self.next_value_node_of(i)
        &&& forall|i: int| #![trigger self.arr_seq@[self.value_list@[i as int] as int].prev] #![trigger self.prev_value_node_of(i)]
            0 <= i < self.value_list@.len() ==> self.arr_seq@[self.value_list@[i as int] as int].prev == self.prev_value_node_of(i)
        &&& forall|i: int| #![trigger self.value_list@[i as int]]
            0 <= i < self.value_list@.len() ==> 0 <= self.value_list@[i as int] < N
        &&& self.unique()
        &&& self.wf_value_node_head()
        &&& self.wf_value_node_tail()
        &&& self.value_list_len == self.value_list@.len()
    }
    pub closed spec fn array_wf(&self) -> bool { &&& self.arr_seq@.len() == N &&& self.size == N }
    pub closed spec fn spec_seq_wf(&self) -> bool {
        &&& self.spec_seq@.len() == self.value_list_len
        &&& forall|i: int| #![trigger self.spec_seq@[i as int]] #![trigger self.value_list@[i as int]]
            0 <= i < self.value_list_len
                ==> self.arr_seq@[self.value_list@[i as int] as int].value.is_Some()
                && self.arr_seq@[self.value_list@[i as int] as int].value.get_Some_0() =~= self.spec_seq@[i as int]
    }
    pub closed spec fn wf(&self) -> bool {
        &&& N <= i32::MAX &&& N > 2 &&& self.array_wf()
        &&& self.free_list_len + self.value_list_len == N
        &&& self.value_list_wf() &&& self.free_list_wf() &&& self.spec_seq_wf()
        &&& forall|i: int, j: int| #![trigger self.value_list@[i], self.free_list@[j]]
            0 <= i < self.value_list@.len() && 0 <= j < self.free_list@.len() ==> self.value_list@[i] != self.free_list@[j]
    }
}

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {
    pub fn remove_helper4(&mut self, remove_index: SLLIndex, v: Ghost<T>) -> (ret: T)
        requires
            old(self).wf(), old(self)@.contains(v@), old(self).get_node_ref(v@) == remove_index,
            old(self).value_list_len != 1,
            old(self).free_list_len == 0 && old(self).value_list_tail != remove_index && old(self).value_list_head != remove_index,
        ensures
            self.wf(), self.len() == old(self).len() - 1, ret == v@,
            forall|v:T| #![auto] self@.contains(v) ==> old(self).get_node_ref(v) == self.get_node_ref(v),
            self.unique(), self@ =~= old(self)@.remove_value(ret),
    {
        proof { seq_push_lemma::<SLLIndex>(); seq_remove_lemma::<SLLIndex>(); seq_remove_lemma::<T>(); }
        let ret = self.get_value(remove_index).unwrap();
        let prev = self.get_prev(remove_index);
        let next = self.get_next(remove_index);
        self.set_next(prev, next);
        self.set_prev(next, prev);
        let ghost_index = Ghost(self.spec_seq@.index_of(v@));
        assert(0 <= ghost_index@ < self.spec_seq@.len());
        assert(self.spec_seq@[ghost_index@] == v@);
        assert(self.value_list@[ghost_index@] == remove_index);
        proof {
            self.value_list@ = self.value_list@.subrange(0, ghost_index@).add(
                self.value_list@.subrange(ghost_index@ + 1, self.value_list_len as int));
            self.spec_seq@ = self.spec_seq@.subrange(0, ghost_index@).add(
                self.spec_seq@.subrange(ghost_index@ + 1, self.value_list_len as int));
        }
        self.value_list_len = self.value_list_len - 1;
        self.free_list_head = remove_index;
        self.free_list_tail = remove_index;
        self.set_prev(remove_index, -1);
        self.set_next(remove_index, -1);
        proof { self.free_list@ = self.free_list@.push(remove_index); }
        self.free_list_len = self.free_list_len + 1;
        assert(self.wf()) by {
            assert(self.array_wf()); assert(self.free_list_len + self.value_list_len == N);
            assert(self.value_list_wf()); assert(self.free_list_wf()); assert(self.spec_seq_wf());
        };
        assert(forall|v:T| #![auto] self@.contains(v) ==> old(self).get_node_ref(v) == self.get_node_ref(v));
        return ret;
    }
}

impl<T: Copy, const N: usize> StaticLinkedList<T, N> {
    #[verifier::external_body] #[verifier(external_body)]
    pub fn set_next(&mut self, index: SLLIndex, v: SLLIndex)
        requires old(self).array_wf(),
        ensures self.array_wf(),
            forall|i: int| #![trigger self.arr_seq@[i]] #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(self).arr_seq@[i],
            self.arr_seq@[index as int].prev == old(self).arr_seq@[index as int].prev,
            self.arr_seq@[index as int].value == old(self).arr_seq@[index as int].value,
            self.arr_seq@[index as int].next == v,
            self.spec_seq@ == old(self).spec_seq@, self.value_list@ == old(self).value_list@,
            self.free_list@ == old(self).free_list@,
            self.value_list_head == old(self).value_list_head, self.value_list_tail == old(self).value_list_tail,
            self.value_list_len == old(self).value_list_len,
            self.free_list_head == old(self).free_list_head, self.free_list_tail == old(self).free_list_tail,
            self.free_list_len == old(self).free_list_len,
    { unimplemented!() }
    #[verifier::external_body] #[verifier(external_body)]
    pub fn set_prev(&mut self, index: SLLIndex, v: SLLIndex)
        requires old(self).array_wf(),
        ensures self.array_wf(),
            forall|i: int| #![trigger self.arr_seq@[i]] #![trigger old(self).arr_seq@[i]]
                0 <= i < self.arr_seq@.len() && i != index ==> self.arr_seq@[i] =~= old(self).arr_seq@[i],
            self.arr_seq@[index as int].next == old(self).arr_seq@[index as int].next,
            self.arr_seq@[index as int].value == old(self).arr_seq@[index as int].value,
            self.arr_seq@[index as int].prev == v,
            self.spec_seq@ == old(self).spec_seq@, self.value_list@ == old(self).value_list@,
            self.free_list@ == old(self).free_list@,
            self.value_list_head == old(self).value_list_head, self.value_list_tail == old(self).value_list_tail,
            self.value_list_len == old(self).value_list_len,
            self.free_list_head == old(self).free_list_head, self.free_list_tail == old(self).free_list_tail,
            self.free_list_len == old(self).free_list_len,
    { unimplemented!() }
    #[verifier::external_body] #[verifier(external_body)]
    pub fn get_value(&self, index: SLLIndex) -> (ret: Option<T>)
        requires 0 <= index < N, self.array_wf(),
        ensures ret == self.arr_seq@[index as int].value,
    { unimplemented!() }
    #[verifier::external_body] #[verifier(external_body)]
    pub fn get_next(&self, index: SLLIndex) -> (next: SLLIndex)
        requires 0 <= index < N, self.array_wf(),
        ensures next == self.arr_seq@[index as int].next,
    { unimplemented!() }
    #[verifier::external_body] #[verifier(external_body)]
    pub fn get_prev(&self, index: SLLIndex) -> (prev: SLLIndex)
        requires 0 <= index < N, self.array_wf(),
        ensures prev == self.arr_seq@[index as int].prev,
    { unimplemented!() }
}

#[verifier::external_body]
pub proof fn seq_push_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A| s.contains(x) ==> s.push(v).contains(v) && s.push(v).contains(x),
        forall|s: Seq<A>, v: A| #![auto] s.push(v).contains(v),
        forall|s: Seq<A>, v: A, x: A| !s.contains(x) && v != x ==> !s.push(v).contains(x),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn seq_remove_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, i: int| #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] != v && s.no_duplicates() ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int)).contains(v),
        forall|s: Seq<A>, v: A, i: int| #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int)).contains(v) == false,
        forall|s: Seq<A>, i: int, j: int| #![trigger s.subrange(0,i), s[j]]
            0 <= i < s.len() && 0 <= j < i ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j] == s[j],
        forall|s: Seq<A>, i: int, j: int| #![trigger s.subrange(0,i), s[j+1]]
            0 <= i < s.len() && i <= j < s.len() - 1 ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j] == s[j + 1],
        forall|s: Seq<A>, v: A, i: int| #![trigger s.remove_value(v), s.subrange(0,i)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int)) == s.remove_value(v),
{ unimplemented!() }


// ==================== COMPLETENESS ROUND 5: CROSS-FUNCTION MISUSE & EDGE CASES ====================
// Tests that misuse function results or test unguaranteed relationships. All should FAIL.

// Test 1: Call remove_helper4 twice - second call should fail (free_list_len != 0)
fn test_double_remove(sll: &mut StaticLinkedList<u64, 5>, idx1: SLLIndex, idx2: SLLIndex, v1: Ghost<u64>, v2: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v1@),
        old(sll).get_node_ref(v1@) == idx1,
        old(sll).value_list_len != 1,
        old(sll).free_list_len == 0 && old(sll).value_list_tail != idx1 && old(sll).value_list_head != idx1,
        v1@ != v2@,
        old(sll)@.contains(v2@),
        old(sll).get_node_ref(v2@) == idx2,
{
    let ret1 = sll.remove_helper4(idx1, v1);
    // Second call should fail: free_list_len is no longer 0 after first removal
    let ret2 = sll.remove_helper4(idx2, v2);
}

// Test 2: Assert remove_helper4 doesn't change node ref of removed element
fn test_removed_node_ref(sll: &mut StaticLinkedList<u64, 5>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len == 0 && old(sll).value_list_tail != remove_index && old(sll).value_list_head != remove_index,
    ensures
        // Spec only guarantees node ref preservation for elements STILL IN the list
        // Claiming it for the removed element is wrong
        old(sll).get_node_ref(v@) == sll.get_node_ref(v@),
{
    let ret = sll.remove_helper4(remove_index, v);
}

// Test 3: Assert remove_helper4 result is contained in old sequence at wrong position
fn test_wrong_position_claim(sll: &mut StaticLinkedList<u64, 5>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len == 0 && old(sll).value_list_tail != remove_index && old(sll).value_list_head != remove_index,
{
    let ret = sll.remove_helper4(remove_index, v);
    // Spec says ret == v@, but doesn't say ret is at index 0
    assert(sll@[0] == ret); // WRONG: ret was removed, so it shouldn't be at index 0
}

// Test 4: Assert seq_push_lemma and seq_remove_lemma cancel out (wrong reasoning)
proof fn test_push_remove_cancel_wrong() {
    seq_push_lemma::<u64>();
    seq_remove_lemma::<u64>();
    let s = Seq::<u64>::empty().push(1u64).push(2u64).push(3u64);
    // Remove 2 (at index 1), then claim 2 is still there
    assert(s.no_duplicates()) by {
        assert forall|i: int, j: int|
            0 <= i < s.len() && 0 <= j < s.len() && i != j
            implies s[i] != s[j]
        by {}
    };
    assert(s.contains(2u64));
    assert(s[1] == 2u64);
    let removed = s.subrange(0, 1).add(s.subrange(2, s.len() as int));
    // Push 4 onto the removed sequence
    let result = removed.push(4u64);
    // Wrongly claim result contains 2 (it doesn't - 2 was removed and 4 was added)
    assert(result.contains(2u64)); // WRONG: 2 was removed, pushing 4 doesn't bring it back
}

// Test 5: Assert that remove_helper4 makes the list empty
fn test_remove_makes_empty(sll: &mut StaticLinkedList<u64, 5>, remove_index: SLLIndex, v: Ghost<u64>)
    requires
        old(sll).wf(),
        old(sll)@.contains(v@),
        old(sll).get_node_ref(v@) == remove_index,
        old(sll).value_list_len != 1,
        old(sll).free_list_len == 0 && old(sll).value_list_tail != remove_index && old(sll).value_list_head != remove_index,
    ensures
        sll@.len() == 0, // WRONG: N > 2, free_list_len was 0, so value_list_len was N > 2, after removal it's N-1 >= 2
{
    let ret = sll.remove_helper4(remove_index, v);
}

}
