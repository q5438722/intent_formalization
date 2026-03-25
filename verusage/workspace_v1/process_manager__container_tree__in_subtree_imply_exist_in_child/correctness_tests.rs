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
    { unimplemented!() }

    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int|
            #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self.len() && 0 <= j < self.len() && i != j ==> self.spec_seq@[i] != self.spec_seq@[j]
    }

    pub open spec fn view(&self) -> Seq<T> { self.spec_seq@ }

    #[verifier::external_body]
    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex
        recommends self.wf(), self@.contains(v),
    { unimplemented!() }

    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
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
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().uppertree_seq@.no_duplicates()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children@.contains(c_ptr) == false
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().subtree_set@.finite()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().uppertree_seq@.len() == container_perms[c_ptr].value().depth
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
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().children@.contains(child_c_ptr)
            ==> container_perms.dom().contains(child_c_ptr)
                && container_perms[child_c_ptr].value().parent.unwrap() == c_ptr
                && container_perms[child_c_ptr].value().depth == container_perms[c_ptr].value().depth + 1
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().parent.is_Some()
            ==> container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())
                && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(c_ptr)
}

pub closed spec fn containers_linkedlist_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent.is_Some() && container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent_rev_ptr.is_Some()
                && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(c_ptr)
                && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children.get_node_ref(c_ptr) == container_perms[c_ptr].value().parent_rev_ptr.unwrap()
}

pub closed spec fn container_childern_depth_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth - 1] == container_perms[c_ptr].value().parent.unwrap()
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
                && container_perms[sub_c_ptr].value().uppertree_seq@.len() > container_perms[c_ptr].value().depth
                && container_perms[sub_c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth as int] == c_ptr
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
                && container_perms[c_ptr].value().uppertree_seq@[container_perms[u_ptr].value().depth as int] == u_ptr
                && container_perms[u_ptr].value().depth == container_perms[c_ptr].value().uppertree_seq@.index_of(u_ptr)
                && container_perms[u_ptr].value().subtree_set@.contains(c_ptr)
                && container_perms[u_ptr].value().uppertree_seq@ =~= container_perms[c_ptr].value().uppertree_seq@.subrange(0, container_perms[u_ptr].value().depth as int)
}

pub closed spec fn container_subtree_set_exclusive(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr, sub_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr), container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms.dom().contains(sub_c_ptr)
            ==> (container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr) == container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr))
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

pub proof fn in_subtree_imply_exist_in_child(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
    ensures
        container_perms[c_ptr].value().children@.contains(s_ptr) || (exists|child_ptr: ContainerPtr|
            #![auto]
            container_perms[c_ptr].value().children@.contains(child_ptr)
                && container_perms[child_ptr].value().subtree_set@.contains(s_ptr)),
{
    assume(false);
}


// ============================================================
// BOUNDARY TESTS (5): Violate preconditions
// ============================================================

// Test B1: Missing container_perms_wf precondition
// SHOULD FAIL
proof fn test_boundary_missing_perms_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
}

// Test B2: Missing container_tree_wf precondition
// SHOULD FAIL
proof fn test_boundary_missing_tree_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
}

// Test B3: c_ptr not in domain
// SHOULD FAIL
proof fn test_boundary_c_ptr_not_in_domain(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        !container_perms.dom().contains(c_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
}

// Test B4: s_ptr not in subtree of c_ptr
// SHOULD FAIL
proof fn test_boundary_s_ptr_not_in_subtree(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        !container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
}

// Test B5: All preconditions missing
// SHOULD FAIL
proof fn test_boundary_no_preconditions(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
}


// ============================================================
// BEHAVIORAL MUTATION TESTS (5): Mutate expected outputs
// ============================================================

// Test M1: Assert s_ptr is ALWAYS a direct child (drops existential branch)
// SHOULD FAIL
proof fn test_mutation_always_direct_child(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
    assert(container_perms[c_ptr].value().children@.contains(s_ptr));
}

// Test M2: Assert s_ptr is NEVER a direct child (negates first disjunct)
// SHOULD FAIL
proof fn test_mutation_never_direct_child(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
    assert(!container_perms[c_ptr].value().children@.contains(s_ptr));
}

// Test M3: Negate conclusion entirely — neither disjunct holds
// SHOULD FAIL
proof fn test_mutation_negate_conclusion(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
    assert(!container_perms[c_ptr].value().children@.contains(s_ptr));
    assert(forall|child_ptr: ContainerPtr|
        #![auto]
        container_perms[c_ptr].value().children@.contains(child_ptr)
            ==> !container_perms[child_ptr].value().subtree_set@.contains(s_ptr));
}

// Test M4: Mediating child must be s_ptr itself (wrong identity)
// SHOULD FAIL
proof fn test_mutation_child_equals_s_ptr(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
        !container_perms[c_ptr].value().children@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
    assert(container_perms[c_ptr].value().children@.contains(s_ptr)
        && container_perms[s_ptr].value().subtree_set@.contains(s_ptr));
}

// Test M5: s_ptr is in ALL children's subtrees (too strong)
// SHOULD FAIL
proof fn test_mutation_in_all_children_subtrees(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    in_subtree_imply_exist_in_child(root_container, container_perms, c_ptr, s_ptr);
    assert(forall|child_ptr: ContainerPtr|
        #![auto]
        container_perms[c_ptr].value().children@.contains(child_ptr)
            ==> container_perms[child_ptr].value().subtree_set@.contains(s_ptr));
}


// ============================================================
// LOGICAL TESTS (5): Properties NOT explicitly guaranteed
// ============================================================

// Test L1: Subtree relationship is symmetric (WRONG — trees are hierarchical)
// SHOULD FAIL
proof fn test_logical_subtree_symmetric(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    assert(container_perms[s_ptr].value().subtree_set@.contains(c_ptr));
}

// Test L2: s_ptr has same depth as c_ptr (WRONG — subtree members are deeper)
// SHOULD FAIL
proof fn test_logical_equal_depth(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    assert(container_perms[s_ptr].value().depth == container_perms[c_ptr].value().depth);
}

// Test L3: c_ptr must be the root container (not guaranteed)
// SHOULD FAIL
proof fn test_logical_c_ptr_is_root(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    assert(c_ptr == root_container);
}

// Test L4: s_ptr depth is EXACTLY c_ptr + 1 (WRONG — could be deeper descendant)
// SHOULD FAIL
proof fn test_logical_depth_exactly_plus_one(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
{
    assert(container_perms[s_ptr].value().depth == container_perms[c_ptr].value().depth + 1);
}

// Test L5: s_ptr cannot be in root's subtree (contradicts tree semantics)
// SHOULD FAIL
proof fn test_logical_not_in_root_subtree(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
    s_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().subtree_set@.contains(s_ptr),
        c_ptr == root_container,
{
    assert(!container_perms[root_container].value().subtree_set@.contains(s_ptr));
}


}
