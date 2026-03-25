// Adversarial proof tests for proc_tree_check_is_ancestor specification
// Tests are organized into three categories:
//   (1) Boundary Tests - violate preconditions
//   (2) Behavioral Mutation Tests - mutate expected outputs
//   (3) Logical Tests - test unintended logical consequences
// ALL tests are expected to FAIL verification (Verus should reject them).

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

#[derive(Clone, Copy, Debug)]
pub enum DemandPagingMode {
    NoDMDPG,
    DirectParentPrc,
    AllParentProc,
    AllParentContainer,
}

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
        proc_perms.dom().contains(p_ptr) ==> proc_perms[p_ptr].value().uppertree_seq@.no_duplicates()
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


// ================================================================
//                    (1) BOUNDARY TESTS
// ================================================================

// BOUNDARY TEST 1: Equal depth (violates depth(a) < depth(child))
// SHOULD FAIL
proof fn boundary_test_equal_depth(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth == proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        a_ptr != child_ptr,
{
    // SHOULD FAIL: equal-depth nodes cannot be ancestors of each other
    reveal(proc_uppertree_seq_wf);
    assert(proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
}

// BOUNDARY TEST 2: Reversed depth (a_ptr deeper than child_ptr)
// SHOULD FAIL
proof fn boundary_test_reversed_depth(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth > proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
{
    // SHOULD FAIL: deeper node cannot be ancestor
    reveal(proc_uppertree_seq_wf);
    assert(proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
}

// BOUNDARY TEST 3: child_ptr is root (violates child_ptr != root_proc)
// SHOULD FAIL
proof fn boundary_test_child_is_root(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(root_proc),
        a_ptr != root_proc,
{
    // SHOULD FAIL: root has depth 0, empty uppertree_seq
    reveal(proc_root_wf);
    assert(proc_perms[root_proc].value().uppertree_seq@.contains(a_ptr));
}

// BOUNDARY TEST 4: a_ptr not in tree domain
// SHOULD FAIL
proof fn boundary_test_a_not_in_domain(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        !proc_tree_dom.contains(a_ptr),
        proc_perms.dom().contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
{
    // SHOULD FAIL: out-of-tree node cannot be guaranteed ancestor
    assert(proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
}


// ================================================================
//                (2) BEHAVIORAL MUTATION TESTS
// ================================================================

// BEHAVIORAL MUTATION TEST 1: Assert ancestor always holds
// SHOULD FAIL
proof fn behavioral_always_ancestor(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
{
    // SHOULD FAIL: lesser depth does not guarantee ancestry
    assert(proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
}

// BEHAVIORAL MUTATION TEST 2: Assert ancestor never holds
// SHOULD FAIL
proof fn behavioral_never_ancestor(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
{
    // SHOULD FAIL: ancestry can hold for valid inputs
    assert(!proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
}

// BEHAVIORAL MUTATION TEST 3: Wrong postcondition (ancestry == direct parent)
// SHOULD FAIL
proof fn behavioral_wrong_postcondition(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
    ret: bool,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        ret == proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // SHOULD FAIL: ancestry != direct parent relationship
    assert(ret == (proc_perms[child_ptr].value().parent == Some(a_ptr)));
}

// BEHAVIORAL MUTATION TEST 4: Ancestor implies direct parent
// SHOULD FAIL
proof fn behavioral_ancestor_implies_parent(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // SHOULD FAIL: ancestor can be grandparent, great-grandparent, etc.
    reveal(proc_uppertree_seq_wf);
    assert(proc_perms[child_ptr].value().parent == Some(a_ptr));
}


// ================================================================
//                     (3) LOGICAL TESTS
// ================================================================

// LOGICAL TEST 1: Symmetry - ancestor is NOT symmetric
// SHOULD FAIL
proof fn logical_test_symmetry(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // SHOULD FAIL: ancestry is asymmetric
    assert(proc_perms[a_ptr].value().uppertree_seq@.contains(child_ptr));
}

// LOGICAL TEST 2: Stronger inequality - ancestor depth is exactly depth-1
// SHOULD FAIL
proof fn logical_test_immediate_depth(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        proc_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // SHOULD FAIL: ancestor can be at any lesser depth, not just depth-1
    reveal(proc_uppertree_seq_wf);
    assert(proc_perms[child_ptr].value().depth == proc_perms[a_ptr].value().depth + 1);
}

// LOGICAL TEST 3: Unique depth - same depth implies same node
// SHOULD FAIL
proof fn logical_test_unique_depth(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    b_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(b_ptr),
        proc_perms[a_ptr].value().depth == proc_perms[b_ptr].value().depth,
{
    // SHOULD FAIL: multiple nodes can share the same depth (siblings)
    assert(a_ptr == b_ptr);
}

// LOGICAL TEST 4: Subtree membership implies direct child
// SHOULD FAIL
proof fn logical_test_subtree_implies_child(
    root_proc: ProcPtr,
    proc_tree_dom: Set<ProcPtr>,
    proc_perms: Map<ProcPtr, PointsTo<Process>>,
    a_ptr: ProcPtr,
    child_ptr: ProcPtr,
)
    requires
        proc_perms_wf(proc_perms),
        proc_tree_wf(root_proc, proc_tree_dom, proc_perms),
        proc_tree_dom_subset_of_proc_dom(proc_tree_dom, proc_perms),
        proc_tree_dom.contains(a_ptr),
        proc_tree_dom.contains(child_ptr),
        proc_perms[a_ptr].value().depth < proc_perms[child_ptr].value().depth,
        child_ptr != root_proc,
        proc_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // SHOULD FAIL: subtree member != direct child
    reveal(proc_subtree_set_wf);
    assert(proc_perms[a_ptr].value().children@.contains(child_ptr));
}


}
