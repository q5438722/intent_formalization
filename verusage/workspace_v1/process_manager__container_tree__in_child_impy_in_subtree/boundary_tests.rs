use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointPtr = usize;

pub type ContainerPtr = usize;
pub type SLLIndex = i32;

pub const NUM_CPUS: usize = 32;

pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;


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



// File: process_manager/container.rs
pub struct Container {
    pub parent: Option<ContainerPtr>,
    pub parent_rev_ptr: Option<SLLIndex>,
    pub children: StaticLinkedList<ContainerPtr, CONTAINER_CHILD_LIST_LEN>,
    pub depth: usize,
    pub uppertree_seq: Ghost<Seq<ContainerPtr>>,
    pub subtree_set: Ghost<Set<ContainerPtr>>,
    pub root_process: Option<ProcPtr>,
    pub owned_procs: StaticLinkedList<ProcPtr, CONTAINER_PROC_LIST_LEN>,
    pub owned_endpoints: Ghost<Set<EndpointPtr>>,
    pub owned_threads: Ghost<Set<ThreadPtr>>,
    pub quota: Quota,
    pub owned_cpus: ArraySet<NUM_CPUS>,
    pub scheduler: StaticLinkedList<ThreadPtr, MAX_CONTAINER_SCHEDULER_LEN>,
    pub can_have_children: bool,
}


// File: array.rs
pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}


// File: array_set.rs
pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,

    pub set: Ghost<Set<usize>>,
}


// File: quota.rs
    #[derive(Clone, Copy, Debug)]
    pub struct Quota{
        pub mem_4k:usize,
        pub mem_2m:usize,
        pub mem_1g:usize,
        pub pcid:usize,
        pub ioid:usize,
    }


// File: process_manager/container_tree.rs
pub open spec fn container_perms_wf(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].is_init()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].addr() == c_ptr
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.wf()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.unique()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().uppertree_seq@.no_duplicates()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children@.contains(
            c_ptr,
        ) == false
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().subtree_set@.finite()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().uppertree_seq@.len()
            == container_perms[c_ptr].value().depth
}

pub closed spec fn container_root_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn container_childern_parent_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn containers_linkedlist_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn container_childern_depth_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn container_subtree_set_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn container_uppertree_seq_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub closed spec fn container_subtree_set_exclusive(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool;

pub open spec fn container_tree_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& container_root_wf(root_container, container_perms)
    &&& container_childern_parent_wf(root_container, container_perms)
    &&& containers_linkedlist_wf(root_container, container_perms)
    &&& container_childern_depth_wf(root_container, container_perms)
    &&& container_subtree_set_wf(root_container, container_perms)
    &&& container_uppertree_seq_wf(root_container, container_perms)
    &&& container_subtree_set_exclusive(root_container, container_perms)
}


// ===== BOUNDARY TESTS =====
// Each test removes one precondition from in_child_impy_in_subtree
// and tries to prove the same postcondition. SHOULD FAIL.

// Test 1: Missing container_perms_wf
// SHOULD FAIL
proof fn test_boundary_missing_perms_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        // container_perms_wf(container_perms),  // REMOVED
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().children@.contains(child_ptr),
        container_perms[child_ptr].value().subtree_set@.contains(s_ptr),
    ensures
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
}

// Test 2: Missing container_tree_wf
// SHOULD FAIL
proof fn test_boundary_missing_tree_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        // container_tree_wf(root_container, container_perms),  // REMOVED
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().children@.contains(child_ptr),
        container_perms[child_ptr].value().subtree_set@.contains(s_ptr),
    ensures
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
}

// Test 3: c_ptr not required to be in domain
// SHOULD FAIL
proof fn test_boundary_c_ptr_not_in_domain(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        // container_perms.dom().contains(c_ptr),  // REMOVED
        container_perms[c_ptr].value().children@.contains(child_ptr),
        container_perms[child_ptr].value().subtree_set@.contains(s_ptr),
    ensures
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
}

// Test 4: child_ptr not required to be a child of c_ptr
// SHOULD FAIL
proof fn test_boundary_child_not_in_children(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        // container_perms[c_ptr].value().children@.contains(child_ptr),  // REMOVED
        container_perms[child_ptr].value().subtree_set@.contains(s_ptr),
    ensures
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
}

// Test 5: s_ptr not required to be in child's subtree
// SHOULD FAIL
proof fn test_boundary_s_ptr_not_in_child_subtree(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().children@.contains(child_ptr),
        // container_perms[child_ptr].value().subtree_set@.contains(s_ptr),  // REMOVED
    ensures
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
}


}
