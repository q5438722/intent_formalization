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

pub open spec fn new_container_ensures(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
) -> bool {
    &&& container_perms_wf(old_container_perms)
    &&& container_perms_wf(new_container_perms)
    &&& container_tree_wf(root_container, old_container_perms)
    &&& old_container_perms.dom().contains(container_ptr)
    &&& old_container_perms.dom().contains(new_container_ptr) == false
    &&& old_container_perms[container_ptr].value().children.len() < PROC_CHILD_LIST_LEN
    &&& old_container_perms[container_ptr].value().depth < usize::MAX
    &&& new_container_perms.dom() == old_container_perms.dom().insert(new_container_ptr)
    &&& new_container_perms[new_container_ptr].value().parent =~= Some(container_ptr)
    &&& new_container_perms[new_container_ptr].value().parent_rev_ptr.is_Some()
    &&& new_container_perms[new_container_ptr].value().children@ =~= Seq::empty()
    &&& new_container_perms[new_container_ptr].value().uppertree_seq@
        =~= old_container_perms[container_ptr].value().uppertree_seq@.push(container_ptr)
    &&& new_container_perms[new_container_ptr].value().depth as int
        =~= old_container_perms[container_ptr].value().depth + 1
    &&& new_container_perms[new_container_ptr].value().uppertree_seq@
        =~= old_container_perms[container_ptr].value().uppertree_seq@.push(container_ptr)
    &&& new_container_perms[new_container_ptr].value().subtree_set@ =~= Set::<ContainerPtr>::empty()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger old_container_perms.dom().contains(c_ptr)]
        old_container_perms.dom().contains(c_ptr) && c_ptr != container_ptr
            ==> new_container_perms[c_ptr].value().parent
            =~= old_container_perms[c_ptr].value().parent
            && new_container_perms[c_ptr].value().parent_rev_ptr
            =~= old_container_perms[c_ptr].value().parent_rev_ptr
            && new_container_perms[c_ptr].value().children
            =~= old_container_perms[c_ptr].value().children
            && new_container_perms[c_ptr].value().depth =~= old_container_perms[c_ptr].value().depth
            && new_container_perms[c_ptr].value().uppertree_seq
            =~= old_container_perms[c_ptr].value().uppertree_seq
    &&& forall|c_ptr: ContainerPtr|
        #![trigger new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr)]
        new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr)
            ==> new_container_perms[c_ptr].value().subtree_set@
            =~= old_container_perms[c_ptr].value().subtree_set@.insert(new_container_ptr)
    &&& forall|c_ptr: ContainerPtr|
        #![trigger old_container_perms.dom().contains(c_ptr)]
        old_container_perms.dom().contains(c_ptr)
            && new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr)
            == false ==> new_container_perms[c_ptr].value().subtree_set
            =~= old_container_perms[c_ptr].value().subtree_set
    &&& new_container_perms[container_ptr].value().parent
        =~= old_container_perms[container_ptr].value().parent
    &&& new_container_perms[container_ptr].value().parent_rev_ptr
        =~= old_container_perms[container_ptr].value().parent_rev_ptr
    &&& new_container_perms[container_ptr].value().children@
        =~= old_container_perms[container_ptr].value().children@.push(new_container_ptr)
    &&& new_container_perms[container_ptr].value().depth
        =~= old_container_perms[container_ptr].value().depth
    &&& new_container_perms[container_ptr].value().uppertree_seq
        =~= old_container_perms[container_ptr].value().uppertree_seq
    &&& new_container_perms[container_ptr].value().children.wf()
    &&& new_container_perms[container_ptr].value().children@
        == old_container_perms[container_ptr].value().children@.push(new_container_ptr)
    &&& new_container_perms[container_ptr].value().children.len()
        == old_container_perms[container_ptr].value().children.len() + 1
    &&&
    forall|v:ContainerPtr|
        #![auto]
        old_container_perms[container_ptr].value().children@.contains(v) ==> 
            old_container_perms[container_ptr].value().children.get_node_ref(v) == 
                new_container_perms[container_ptr].value().children.get_node_ref(v)
    &&&
    new_container_perms[container_ptr].value().children.get_node_ref(new_container_ptr) == 
        new_container_perms[new_container_ptr].value().parent_rev_ptr.unwrap()
    &&& new_container_perms[container_ptr].value().children.unique()
}

pub const PROC_CHILD_LIST_LEN: usize = 10;


// ============================================================
// LOGICAL TESTS: Properties NOT explicitly guaranteed by the spec
// Each test assumes new_container_ensures holds and tries to derive
// a property that is plausible but not entailed. These SHOULD FAIL.
// ============================================================

// SHOULD FAIL: new container's root_process is None (not specified by ensures)
// The spec says nothing about the root_process field of the new container
proof fn logical_test_root_process_none(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires
        new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    // The spec does not specify root_process for the new container
    assert(new_container_perms[new_container_ptr].value().root_process.is_None()); // SHOULD FAIL
}

// SHOULD FAIL: new container's can_have_children is false (not specified)
// The spec does not constrain this field for the new container
proof fn logical_test_can_have_children_false(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires
        new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    // The spec never constrains can_have_children for the new container
    assert(new_container_perms[new_container_ptr].value().can_have_children == false); // SHOULD FAIL
}

// SHOULD FAIL: new container's owned_endpoints is empty (not specified)
// The spec does not mention owned_endpoints for the new container
proof fn logical_test_owned_endpoints_empty(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires
        new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    // The spec never constrains owned_endpoints for the new container
    assert(new_container_perms[new_container_ptr].value().owned_endpoints@ =~= Set::<EndpointPtr>::empty()); // SHOULD FAIL
}

// SHOULD FAIL: parent container's quota is preserved (not specified)
// The spec preserves parent, depth, children, uppertree_seq but says nothing about quota
proof fn logical_test_parent_quota_preserved(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires
        new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    // The spec preserves structural fields but NOT quota for the parent container
    assert(new_container_perms[container_ptr].value().quota.mem_4k
        == old_container_perms[container_ptr].value().quota.mem_4k); // SHOULD FAIL
}

// SHOULD FAIL: non-parent container's owned_threads is preserved (not specified for non-structural fields)
// For c_ptr != container_ptr, the spec preserves parent/children/depth/uppertree_seq but NOT owned_threads
proof fn logical_test_sibling_owned_threads_preserved(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
    other_ptr: ContainerPtr,
)
    requires
        new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
        old_container_perms.dom().contains(other_ptr),
        other_ptr != container_ptr,
        other_ptr != new_container_ptr,
{
    // The spec preserves structural fields for other_ptr but NOT owned_threads
    assert(new_container_perms[other_ptr].value().owned_threads@
        =~= old_container_perms[other_ptr].value().owned_threads@); // SHOULD FAIL
}

}
