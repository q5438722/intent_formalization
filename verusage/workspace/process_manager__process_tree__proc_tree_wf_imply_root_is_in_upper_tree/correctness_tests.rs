use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type IOid = usize;

pub type ThreadPtr = usize;

pub type ProcPtr = usize;

pub type ContainerPtr = usize;

pub type Pcid = usize;

pub type SLLIndex = i32;

pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;

pub type PagePerm2m = PointsTo<[u8; PAGE_SZ_2m]>;

pub type PagePerm1g = PointsTo<[u8; PAGE_SZ_1g]>;

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;

pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const PAGE_SZ_2m: usize = 1usize << 21;

pub const PAGE_SZ_1g: usize = 1usize << 30;

pub const PROC_CHILD_LIST_LEN: usize = 10;


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

	#[verifier::external_body]
    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex
        recommends
            self.wf(),
            self@.contains(v),
	{
		unimplemented!()
	}

	#[verifier::external_body]
    pub closed spec fn wf(&self) -> bool {
		unimplemented!()
	}


}



// File: process_manager/process.rs
pub struct Process {
    pub owning_container: ContainerPtr,
    pub rev_ptr: SLLIndex,
    pub pcid: Pcid,
    pub ioid: Option<IOid>,
    pub owned_threads: StaticLinkedList<ThreadPtr, MAX_NUM_THREADS_PER_PROC>,
    pub parent: Option<ProcPtr>,
    pub parent_rev_ptr: Option<SLLIndex>,
    pub children: StaticLinkedList<ProcPtr, PROC_CHILD_LIST_LEN>,
    pub uppertree_seq: Ghost<Seq<ProcPtr>>,
    pub subtree_set: Ghost<Set<ProcPtr>>,
    pub depth: usize,
    pub dmd_paging_mode: DemandPagingMode,
}


// File: define.rs
#[derive(Clone, Copy, Debug)]
pub enum DemandPagingMode {
    NoDMDPG,
    DirectParentPrc,
    AllParentProc,
    AllParentContainer,
}


// File: process_manager/process_tree.rs
pub open spec fn proc_perms_wf(proc_perms: Map<ProcPtr, PointsTo<Process>>) -> bool {
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].is_init()
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].addr() == p_ptr
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().children.wf()
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().children.unique()
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr)
            ==> proc_perms[p_ptr].value().uppertree_seq@.no_duplicates()
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().children@.contains(p_ptr)
            == false
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().subtree_set@.finite()
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_perms.dom().contains(p_ptr)]
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().uppertree_seq@.len()
            == proc_perms[p_ptr].value().depth
}

pub open spec fn proc_tree_dom_subset_of_proc_dom(
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) ==> proc_perms.dom().contains(p_ptr)
}

pub closed spec fn proc_root_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& proc_tree_dom.contains(root_proc)
    &&& proc_perms[root_proc].value().depth == 0
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && p_ptr != root_proc ==> proc_perms[p_ptr].value().depth != 0
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && p_ptr != root_proc
            ==> proc_perms[p_ptr].value().parent.is_Some()
}

pub closed spec fn proc_childern_parent_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr, child_p_ptr: ProcPtr|
        #![trigger proc_perms[p_ptr].value().children@.contains(child_p_ptr)]
        proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().children@.contains(child_p_ptr)
            ==> proc_tree_dom.contains(child_p_ptr)
            && proc_perms[child_p_ptr].value().parent.unwrap() == p_ptr
            && proc_perms[child_p_ptr].value().depth == proc_perms[p_ptr].value().depth + 1
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().parent.is_Some()
            ==> proc_tree_dom.contains(proc_perms[p_ptr].value().parent.unwrap())
            && proc_perms[proc_perms[p_ptr].value().parent.unwrap()].value().children@.contains(
            p_ptr,
        )
}

pub closed spec fn procs_linkedlist_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(proc_perms[p_ptr].value().parent.unwrap())]
        proc_tree_dom.contains(p_ptr) && p_ptr != root_proc
            ==> proc_perms[p_ptr].value().parent.is_Some() && proc_tree_dom.contains(
            proc_perms[p_ptr].value().parent.unwrap(),
        )
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && p_ptr != root_proc
            ==> proc_perms[p_ptr].value().parent_rev_ptr.is_Some()
            && proc_perms[proc_perms[p_ptr].value().parent.unwrap()].value().children@.contains(
            p_ptr,
        ) && proc_perms[proc_perms[p_ptr].value().parent.unwrap()].value().children.get_node_ref(p_ptr)
        ==
        proc_perms[p_ptr].value().parent_rev_ptr.unwrap()
}

pub closed spec fn proc_childern_depth_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr|
        #![trigger proc_tree_dom.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && p_ptr != root_proc
            ==> proc_perms[p_ptr].value().uppertree_seq@[proc_perms[p_ptr].value().depth - 1]
            == proc_perms[p_ptr].value().parent.unwrap()
}

pub closed spec fn proc_subtree_set_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr, sub_p_ptr: ProcPtr|
        #![trigger proc_perms[p_ptr].value().subtree_set@.contains(sub_p_ptr)]
        proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().subtree_set@.contains(sub_p_ptr)
            ==> proc_tree_dom.contains(sub_p_ptr)
            && proc_perms[sub_p_ptr].value().uppertree_seq@.len() > proc_perms[p_ptr].value().depth
            && proc_perms[sub_p_ptr].value().uppertree_seq@[proc_perms[p_ptr].value().depth as int]
            == p_ptr
}

pub closed spec fn proc_uppertree_seq_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr, u_ptr: ProcPtr|
        #![trigger proc_perms[p_ptr].value().uppertree_seq@.contains(u_ptr)]
        proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().uppertree_seq@.contains(u_ptr)
            ==> proc_tree_dom.contains(u_ptr)
            && proc_perms[p_ptr].value().uppertree_seq@[proc_perms[u_ptr].value().depth as int]
            == u_ptr && proc_perms[u_ptr].value().depth
            == proc_perms[p_ptr].value().uppertree_seq@.index_of(u_ptr)
            && proc_perms[u_ptr].value().subtree_set@.contains(p_ptr)
            && proc_perms[u_ptr].value().uppertree_seq@
            =~= proc_perms[p_ptr].value().uppertree_seq@.subrange(
            0,
            proc_perms[u_ptr].value().depth as int,
        )
}

pub closed spec fn proc_subtree_set_exclusive(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& forall|p_ptr: ProcPtr, sub_p_ptr: ProcPtr|
        #![trigger proc_perms[p_ptr].value().subtree_set@.contains(sub_p_ptr), proc_perms[sub_p_ptr].value().uppertree_seq@.contains(p_ptr)]
        proc_tree_dom.contains(p_ptr) && proc_tree_dom.contains(sub_p_ptr) ==> (
        proc_perms[p_ptr].value().subtree_set@.contains(sub_p_ptr)
            == proc_perms[sub_p_ptr].value().uppertree_seq@.contains(p_ptr))
}

pub open spec fn proc_tree_wf(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
) -> bool {
    &&& proc_root_wf(root_proc, proc_tree_dom, proc_perms)
    &&& proc_childern_parent_wf(root_proc, proc_tree_dom, proc_perms)
    &&& procs_linkedlist_wf(root_proc, proc_tree_dom, proc_perms)
    &&& proc_childern_depth_wf(root_proc, proc_tree_dom, proc_perms)
    &&& proc_subtree_set_wf(root_proc, proc_tree_dom, proc_perms)
    &&& proc_uppertree_seq_wf(root_proc, proc_tree_dom, proc_perms)
    &&& proc_subtree_set_exclusive(root_proc, proc_tree_dom, proc_perms)
}

    #[verifier::spinoff_prover]
pub proof fn proc_tree_wf_imply_root_is_in_upper_tree(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),

    ensures
         forall|p_ptr: ProcPtr|
            #![auto]
            proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().depth != 0
            ==>
            proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc,
{
    assert(
        forall|p_ptr: ProcPtr|
            #![auto]
            proc_tree_dom.contains(p_ptr) && proc_perms[p_ptr].value().depth != 0
            ==>
            proc_perms[p_ptr].value().uppertree_seq@.contains(proc_perms[p_ptr].value().uppertree_seq@[0])
    );
}

// ==================== CORRECTNESS TESTS ====================
// These tests should all PASS (verify successfully).
// They verify that the lemma's postcondition holds when preconditions are met.

// Test 1: Basic parameterized test - call the lemma and check the postcondition
proof fn test_param_basic(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth != 0,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
}

// Test 2: Verify the universal quantifier holds for two arbitrary distinct processes
proof fn test_param_two_processes(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p1: ProcPtr,
    p2: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p1),
        proc_tree_dom.contains(p2),
        proc_perms[p1].value().depth != 0,
        proc_perms[p2].value().depth != 0,
        p1 != p2,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    // Both should have root as first element of uppertree_seq
    assert(proc_perms[p1].value().uppertree_seq@[0] == root_proc);
    assert(proc_perms[p2].value().uppertree_seq@[0] == root_proc);
    // They share the same root
    assert(proc_perms[p1].value().uppertree_seq@[0] == proc_perms[p2].value().uppertree_seq@[0]);
}

// Test 3: Postcondition is vacuously true for root (depth == 0 case)
proof fn test_param_root_vacuous(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    // The postcondition is a universal with antecedent depth != 0,
    // so it doesn't assert anything about root_proc itself (depth == 0).
    // This test just verifies the lemma can be called and returns.
}

// Test 4: Verify the implication structure - if depth == 0, the consequent doesn't need to hold
proof fn test_param_depth_zero_implication(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth == 0,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    // The lemma's postcondition has antecedent depth != 0, so no assertion about p_ptr is implied.
    // This test just confirms the lemma is callable; no crash or panic.
}

// Test 5: Verify the postcondition holds for a process with depth == 1 (direct child of root)
proof fn test_param_depth_one(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth == 1,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    assert(proc_perms[p_ptr].value().depth != 0);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
}

// Test 6: Verify the postcondition holds for a deeply nested process
proof fn test_param_deep_process(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth > 5,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    assert(proc_perms[p_ptr].value().depth != 0);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
}

// Test 7: Calling the lemma multiple times is idempotent
proof fn test_param_idempotent(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth != 0,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
    // Call again - should still be fine
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
}

// Test 8: Verify postcondition combined with proc_perms_wf knowledge
// uppertree_seq length == depth, and depth != 0, so uppertree_seq is non-empty
proof fn test_param_uppertree_nonempty(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    p_ptr: ProcPtr,
)
    requires
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom.contains(p_ptr),
        proc_perms[p_ptr].value().depth != 0,
{
    proc_tree_wf_imply_root_is_in_upper_tree(root_proc, proc_tree_dom, proc_perms);
    // From proc_perms_wf: uppertree_seq@.len() == depth, and depth != 0
    assert(proc_perms[p_ptr].value().uppertree_seq@.len() > 0);
    assert(proc_perms[p_ptr].value().uppertree_seq@[0] == root_proc);
}


}
