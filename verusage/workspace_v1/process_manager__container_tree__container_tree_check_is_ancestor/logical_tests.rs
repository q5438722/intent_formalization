use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

global size_of usize == 8;    

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointPtr = usize;

pub type ContainerPtr = usize;
pub type SLLIndex = i32;

pub const NUM_CPUS: usize = 32;

pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;


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

pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,
    pub set: Ghost<Set<usize>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Quota{
    pub mem_4k:usize,
    pub mem_2m:usize,
    pub mem_1g:usize,
    pub pcid:usize,
    pub ioid:usize,
}


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
) -> bool {
    &&& container_perms.dom().contains(root_container)
    &&& container_perms[root_container].value().depth == 0
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().depth != 0
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent.is_Some()
}

pub closed spec fn container_childern_parent_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr, child_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().children@.contains(child_c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().children@.contains(
            child_c_ptr,
        ) ==> container_perms.dom().contains(child_c_ptr)
            && container_perms[child_c_ptr].value().parent.unwrap() == c_ptr
            && container_perms[child_c_ptr].value().depth == container_perms[c_ptr].value().depth
            + 1
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().parent.is_Some()
            ==> container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(
        c_ptr)
}

pub closed spec fn containers_linkedlist_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent.is_Some() && container_perms.dom().contains(
            container_perms[c_ptr].value().parent.unwrap(),
        )
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent_rev_ptr.is_Some()
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(
        c_ptr)
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children.get_node_ref(c_ptr) == 
        container_perms[c_ptr].value().parent_rev_ptr.unwrap()
}

pub closed spec fn container_childern_depth_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth
            - 1] == container_perms[c_ptr].value().parent.unwrap()
}

pub closed spec fn container_subtree_set_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr, sub_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)]
        container_perms.dom().contains(c_ptr)
            && container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)
            ==> container_perms.dom().contains(sub_c_ptr)
            && container_perms[sub_c_ptr].value().uppertree_seq@.len()
            > container_perms[c_ptr].value().depth
            && container_perms[sub_c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth as int]
            == c_ptr
}

pub closed spec fn container_uppertree_seq_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr, u_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().uppertree_seq@.contains(u_ptr)]
        container_perms.dom().contains(c_ptr)
            && container_perms[c_ptr].value().uppertree_seq@.contains(u_ptr)
            ==> container_perms.dom().contains(u_ptr)
            && container_perms[c_ptr].value().uppertree_seq@[container_perms[u_ptr].value().depth as int]
            == u_ptr && container_perms[u_ptr].value().depth
            == container_perms[c_ptr].value().uppertree_seq@.index_of(u_ptr)
            && container_perms[u_ptr].value().subtree_set@.contains(c_ptr)
            && container_perms[u_ptr].value().uppertree_seq@
            =~= container_perms[c_ptr].value().uppertree_seq@.subrange(
            0,
            container_perms[u_ptr].value().depth as int,
        )
}

pub closed spec fn container_subtree_set_exclusive(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr, sub_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr), container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms.dom().contains(sub_c_ptr) ==> (
        container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)
            == container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr))
}

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


// ===== LOGICAL TESTS =====
// Each test asserts a property NOT explicitly guaranteed by the specification.
// These should ALL fail verification.

// Test 1: Symmetry — if a_ptr is an ancestor of child_ptr, then child_ptr is also
// an ancestor of a_ptr. Ancestry is directional, NOT symmetric.
// SHOULD FAIL
proof fn test_logical_symmetry(
    root: ContainerPtr,
    perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(perms),
        container_tree_wf(root, perms),
        perms.dom().contains(a_ptr),
        perms.dom().contains(child_ptr),
        perms[a_ptr].value().depth < perms[child_ptr].value().depth,
        perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // Ancestry is NOT symmetric
    assert(perms[a_ptr].value().uppertree_seq@.contains(child_ptr));
}

// Test 2: Every node at a greater depth is a descendant.
// Depth ordering does NOT imply ancestry.
// SHOULD FAIL
proof fn test_logical_all_deeper_are_descendants(
    root: ContainerPtr,
    perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(perms),
        container_tree_wf(root, perms),
        perms.dom().contains(a_ptr),
        perms.dom().contains(child_ptr),
        perms[a_ptr].value().depth < perms[child_ptr].value().depth,
{
    // Not every deeper node is in the subtree
    assert(perms[a_ptr].value().subtree_set@.contains(child_ptr));
}

// Test 3: Ancestry requires a depth gap of at least 2.
// A direct parent-child pair has depth gap of exactly 1, which suffices.
// SHOULD FAIL
proof fn test_logical_depth_gap_at_least_2(
    root: ContainerPtr,
    perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(perms),
        container_tree_wf(root, perms),
        perms.dom().contains(a_ptr),
        perms.dom().contains(child_ptr),
        perms[a_ptr].value().depth < perms[child_ptr].value().depth,
        perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
{
    // Ancestry can exist with depth difference of 1 (parent-child)
    assert(perms[child_ptr].value().depth >= perms[a_ptr].value().depth + 2);
}

// Test 4: Self-ancestry — a node is its own ancestor.
// A node should NOT be in its own uppertree_seq (no cycles).
// SHOULD FAIL
proof fn test_logical_self_ancestry(
    root: ContainerPtr,
    perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(perms),
        container_tree_wf(root, perms),
        perms.dom().contains(c_ptr),
{
    // A node should NOT be its own ancestor
    assert(perms[c_ptr].value().uppertree_seq@.contains(c_ptr));
}

// Test 5: Being in the subtree implies being a direct child.
// Subtree includes all descendants (grandchildren, etc.), not just direct children.
// SHOULD FAIL
proof fn test_logical_subtree_implies_direct_child(
    root: ContainerPtr,
    perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(perms),
        container_tree_wf(root, perms),
        perms.dom().contains(a_ptr),
        perms.dom().contains(child_ptr),
        perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // Subtree membership does NOT imply direct child
    assert(perms[a_ptr].value().children@.contains(child_ptr));
}


}
