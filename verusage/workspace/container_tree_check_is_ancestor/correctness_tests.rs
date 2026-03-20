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


// ============================================================
// CORRECTNESS TESTS - All should PASS
// ============================================================

// --- seq_push_lemma tests ---

// Test: pushed value is contained in the resulting sequence
proof fn test_seq_push_contains_pushed() {
    seq_push_lemma::<usize>();
    let s: Seq<usize> = Seq::empty();
    assert(s.push(42usize).contains(42usize));
}

// Test: existing contained values are preserved after push
proof fn test_seq_push_preserves_existing(s: Seq<usize>, x: usize, v: usize)
    requires
        s.contains(x),
{
    seq_push_lemma::<usize>();
    assert(s.push(v).contains(x));
}

// Test: non-contained different elements remain non-contained after push
proof fn test_seq_push_not_contained_stays(s: Seq<usize>, x: usize, v: usize)
    requires
        !s.contains(x),
        v != x,
{
    seq_push_lemma::<usize>();
    assert(!s.push(v).contains(x));
}

// Test: pushed value is always contained regardless of initial sequence
proof fn test_seq_push_always_contains_pushed(s: Seq<usize>, v: usize) {
    seq_push_lemma::<usize>();
    assert(s.push(v).contains(v));
}

// --- container_perms_wf tests (open spec - directly verifiable) ---

// Test: container_perms_wf implies containers in domain are initialized
proof fn test_perms_wf_is_init(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].is_init());
}

// Test: container_perms_wf implies addr matches the pointer
proof fn test_perms_wf_addr(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].addr() == c_ptr);
}

// Test: container_perms_wf implies children list is well-formed
proof fn test_perms_wf_children_wf(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().children.wf());
}

// Test: container_perms_wf implies children list has unique elements
proof fn test_perms_wf_children_unique(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().children.unique());
}

// Test: container_perms_wf implies uppertree_seq has no duplicates
proof fn test_perms_wf_uppertree_no_duplicates(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().uppertree_seq@.no_duplicates());
}

// Test: container_perms_wf implies a container is not its own child
proof fn test_perms_wf_not_self_child(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().children@.contains(c_ptr) == false);
}

// Test: container_perms_wf implies subtree set is finite
proof fn test_perms_wf_subtree_finite(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().subtree_set@.finite());
}

// Test: container_perms_wf implies depth equals uppertree_seq length
proof fn test_perms_wf_depth_eq_uppertree_len(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_perms.dom().contains(c_ptr),
{
    assert(container_perms[c_ptr].value().uppertree_seq@.len() == container_perms[c_ptr].value().depth);
}

// --- container_tree_wf tests (open spec - extracts closed components) ---

// Test: container_tree_wf implies container_root_wf
proof fn test_tree_wf_implies_root_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_root_wf(root_container, container_perms));
}

// Test: container_tree_wf implies container_subtree_set_exclusive
proof fn test_tree_wf_implies_subtree_exclusive(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_subtree_set_exclusive(root_container, container_perms));
}

// Test: container_tree_wf implies container_subtree_set_wf
proof fn test_tree_wf_implies_subtree_set_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_subtree_set_wf(root_container, container_perms));
}

// Test: container_tree_wf implies container_uppertree_seq_wf
proof fn test_tree_wf_implies_uppertree_seq_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_uppertree_seq_wf(root_container, container_perms));
}

// Test: container_tree_wf implies container_childern_parent_wf
proof fn test_tree_wf_implies_children_parent_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_childern_parent_wf(root_container, container_perms));
}

// Test: container_tree_wf implies containers_linkedlist_wf
proof fn test_tree_wf_implies_linkedlist_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(containers_linkedlist_wf(root_container, container_perms));
}

// Test: container_tree_wf implies container_childern_depth_wf
proof fn test_tree_wf_implies_depth_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_tree_wf(root_container, container_perms),
{
    assert(container_childern_depth_wf(root_container, container_perms));
}

// --- Reveal closed specs and test their internal properties ---

// Test: reveal container_root_wf: root is in domain and has depth 0
proof fn test_root_wf_root_depth_zero(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
)
    requires
        container_root_wf(root_container, container_perms),
{
    reveal(container_root_wf);
    assert(container_perms.dom().contains(root_container));
    assert(container_perms[root_container].value().depth == 0);
}

// Test: reveal container_root_wf: non-root node has non-zero depth
proof fn test_root_wf_nonroot_nonzero_depth(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_root_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        c_ptr != root_container,
{
    reveal(container_root_wf);
    assert(container_perms[c_ptr].value().depth != 0);
}

// Test: reveal container_root_wf: non-root node has a parent
proof fn test_root_wf_nonroot_has_parent(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_root_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        c_ptr != root_container,
{
    reveal(container_root_wf);
    assert(container_perms[c_ptr].value().parent.is_Some());
}

// --- Function ensures consistency tests ---

// Test: the two ensures of container_tree_check_is_ancestor are consistent
// (assuming both ensures hold, they agree on the return value)
proof fn test_ensures_consistency(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
    ret: bool,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth < container_perms[child_ptr].value().depth,
        ret == container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
        ret == container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // Both ensures agree: uppertree_seq.contains(a_ptr) <==> subtree_set.contains(child_ptr)
    assert(container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr)
        == container_perms[a_ptr].value().subtree_set@.contains(child_ptr));
}

// Test: if return is true, both containment relations hold
proof fn test_ensures_true_implies_containment(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth < container_perms[child_ptr].value().depth,
        // Assume the ensures with ret=true
        container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
        container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    assert(container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
    assert(container_perms[a_ptr].value().subtree_set@.contains(child_ptr));
}

// Test: if return is false, both non-containment relations hold
proof fn test_ensures_false_implies_non_containment(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth < container_perms[child_ptr].value().depth,
        // Assume the ensures with ret=false
        !container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr),
        !container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    assert(!container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr));
    assert(!container_perms[a_ptr].value().subtree_set@.contains(child_ptr));
}

}
