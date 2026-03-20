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

pub struct Array<A, const N: usize>{ pub seq: Ghost<Seq<A>>, pub ar: [A;N] }
pub struct ArraySet<const N: usize> { pub data: Array<bool, N>, pub len: usize, pub set: Ghost<Set<usize>> }

#[derive(Clone, Copy, Debug)]
pub struct Quota{ pub mem_4k:usize, pub mem_2m:usize, pub mem_1g:usize, pub pcid:usize, pub ioid:usize }

pub open spec fn container_perms_wf(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].is_init()
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].addr() == c_ptr
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.wf()
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.unique()
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().uppertree_seq@.no_duplicates()
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children@.contains(c_ptr) == false
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().subtree_set@.finite()
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().uppertree_seq@.len() == container_perms[c_ptr].value().depth
}

pub closed spec fn container_root_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& container_perms.dom().contains(root_container)
    &&& container_perms[root_container].value().depth == 0
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container ==> container_perms[c_ptr].value().depth != 0
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container ==> container_perms[c_ptr].value().parent.is_Some()
}

pub closed spec fn container_childern_parent_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr, child_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().children@.contains(child_c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().children@.contains(child_c_ptr)
            ==> container_perms.dom().contains(child_c_ptr)
            && container_perms[child_c_ptr].value().parent.unwrap() == c_ptr
            && container_perms[child_c_ptr].value().depth == container_perms[c_ptr].value().depth + 1
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().parent.is_Some()
            ==> container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(c_ptr)
}

pub closed spec fn containers_linkedlist_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent.is_Some()
            && container_perms.dom().contains(container_perms[c_ptr].value().parent.unwrap())
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().parent_rev_ptr.is_Some()
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children@.contains(c_ptr)
            && container_perms[container_perms[c_ptr].value().parent.unwrap()].value().children.get_node_ref(c_ptr)
                == container_perms[c_ptr].value().parent_rev_ptr.unwrap()
}

pub closed spec fn container_childern_depth_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr| #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && c_ptr != root_container
            ==> container_perms[c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth - 1]
                == container_perms[c_ptr].value().parent.unwrap()
}

pub closed spec fn container_subtree_set_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr, sub_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)
            ==> container_perms.dom().contains(sub_c_ptr)
            && container_perms[sub_c_ptr].value().uppertree_seq@.len() > container_perms[c_ptr].value().depth
            && container_perms[sub_c_ptr].value().uppertree_seq@[container_perms[c_ptr].value().depth as int] == c_ptr
}

pub closed spec fn container_uppertree_seq_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr, u_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().uppertree_seq@.contains(u_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms[c_ptr].value().uppertree_seq@.contains(u_ptr)
            ==> container_perms.dom().contains(u_ptr)
            && container_perms[c_ptr].value().uppertree_seq@[container_perms[u_ptr].value().depth as int] == u_ptr
            && container_perms[u_ptr].value().depth == container_perms[c_ptr].value().uppertree_seq@.index_of(u_ptr)
            && container_perms[u_ptr].value().subtree_set@.contains(c_ptr)
            && container_perms[u_ptr].value().uppertree_seq@
                =~= container_perms[c_ptr].value().uppertree_seq@.subrange(0, container_perms[u_ptr].value().depth as int)
}

pub closed spec fn container_subtree_set_exclusive(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
    &&& forall|c_ptr: ContainerPtr, sub_c_ptr: ContainerPtr|
        #![trigger container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr), container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr)]
        container_perms.dom().contains(c_ptr) && container_perms.dom().contains(sub_c_ptr)
            ==> (container_perms[c_ptr].value().subtree_set@.contains(sub_c_ptr)
                == container_perms[sub_c_ptr].value().uppertree_seq@.contains(c_ptr))
}

pub open spec fn container_tree_wf(root_container: ContainerPtr, container_perms: Map<ContainerPtr, PointsTo<Container>>) -> bool {
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
    &&& new_container_perms[new_container_ptr].value().subtree_set@ =~= Set::<ContainerPtr>::empty()
    &&& forall|c_ptr: ContainerPtr| #![trigger old_container_perms.dom().contains(c_ptr)]
        old_container_perms.dom().contains(c_ptr) && c_ptr != container_ptr
            ==> new_container_perms[c_ptr].value().parent =~= old_container_perms[c_ptr].value().parent
            && new_container_perms[c_ptr].value().parent_rev_ptr =~= old_container_perms[c_ptr].value().parent_rev_ptr
            && new_container_perms[c_ptr].value().children =~= old_container_perms[c_ptr].value().children
            && new_container_perms[c_ptr].value().depth =~= old_container_perms[c_ptr].value().depth
            && new_container_perms[c_ptr].value().uppertree_seq =~= old_container_perms[c_ptr].value().uppertree_seq
    &&& forall|c_ptr: ContainerPtr|
        #![trigger new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr)]
        new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr)
            ==> new_container_perms[c_ptr].value().subtree_set@
            =~= old_container_perms[c_ptr].value().subtree_set@.insert(new_container_ptr)
    &&& forall|c_ptr: ContainerPtr| #![trigger old_container_perms.dom().contains(c_ptr)]
        old_container_perms.dom().contains(c_ptr)
            && new_container_perms[new_container_ptr].value().uppertree_seq@.contains(c_ptr) == false
            ==> new_container_perms[c_ptr].value().subtree_set =~= old_container_perms[c_ptr].value().subtree_set
    &&& new_container_perms[container_ptr].value().parent =~= old_container_perms[container_ptr].value().parent
    &&& new_container_perms[container_ptr].value().parent_rev_ptr =~= old_container_perms[container_ptr].value().parent_rev_ptr
    &&& new_container_perms[container_ptr].value().children@
        =~= old_container_perms[container_ptr].value().children@.push(new_container_ptr)
    &&& new_container_perms[container_ptr].value().depth =~= old_container_perms[container_ptr].value().depth
    &&& new_container_perms[container_ptr].value().uppertree_seq =~= old_container_perms[container_ptr].value().uppertree_seq
    &&& new_container_perms[container_ptr].value().children.wf()
    &&& new_container_perms[container_ptr].value().children@ == old_container_perms[container_ptr].value().children@.push(new_container_ptr)
    &&& new_container_perms[container_ptr].value().children.len() == old_container_perms[container_ptr].value().children.len() + 1
    &&& forall|v:ContainerPtr| #![auto]
        old_container_perms[container_ptr].value().children@.contains(v)
            ==> old_container_perms[container_ptr].value().children.get_node_ref(v)
                == new_container_perms[container_ptr].value().children.get_node_ref(v)
    &&& new_container_perms[container_ptr].value().children.get_node_ref(new_container_ptr)
        == new_container_perms[new_container_ptr].value().parent_rev_ptr.unwrap()
    &&& new_container_perms[container_ptr].value().children.unique()
}

pub proof fn new_container_preserve_tree_inv_4(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
    ensures container_subtree_set_wf(root_container, new_container_perms),
{
    seq_push_lemma::<ContainerPtr>();
    seq_push_unique_lemma::<ContainerPtr>();
}

#[verifier::external_body]
pub proof fn seq_push_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A| s.contains(x) ==> s.push(v).contains(v) && s.push(v).contains(x),
        forall|s: Seq<A>, v: A| #![auto] s.push(v).contains(v),
        forall|s: Seq<A>, v: A, x: A| !s.contains(x) && v != x ==> !s.push(v).contains(x),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn seq_push_unique_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A| #![auto]
            s.no_duplicates() && s.contains(v) == false ==> s.push(v).no_duplicates() && s.push(v).index_of(v) == s.push(v).len() - 1,
        forall|s: Seq<A>, v: A, y: A| #![auto]
            s.no_duplicates() && s.contains(v) && s.contains(y) == false ==> s.push(y).index_of(v) == s.index_of(v),
{ unimplemented!() }

pub const PROC_CHILD_LIST_LEN: usize = 10;


// ==================== CORRECTNESS TESTS (should all PASS) ====================

// Test 1: Basic postcondition - call lemma and assert its ensures
proof fn test_param_basic_postcondition(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    new_container_preserve_tree_inv_4(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr);
    assert(container_subtree_set_wf(root_container, new_container_perms));
}

// Test 2: Old tree well-formedness from precondition (open spec decomposition)
proof fn test_old_tree_wf(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(container_tree_wf(root_container, old_container_perms));
}

// Test 3: Old subtree_set_wf from precondition (component of open container_tree_wf)
proof fn test_old_subtree_set_wf(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(container_subtree_set_wf(root_container, old_container_perms));
}

// Test 4: container_perms_wf for both old and new
proof fn test_perms_wf(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(container_perms_wf(old_container_perms));
    assert(container_perms_wf(new_container_perms));
}

// Test 5: Domain relationships
proof fn test_domain_relationship(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms.dom() == old_container_perms.dom().insert(new_container_ptr));
    assert(old_container_perms.dom().contains(container_ptr));
    assert(!old_container_perms.dom().contains(new_container_ptr));
}

// Test 6: New container has correct parent
proof fn test_new_container_parent(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().parent =~= Some(container_ptr));
}

// Test 7: New container has empty children
proof fn test_new_container_empty_children(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().children@ =~= Seq::empty());
}

// Test 8: New container depth = parent depth + 1
proof fn test_new_container_depth(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().depth as int
        =~= old_container_perms[container_ptr].value().depth + 1);
}

// Test 9: New container has empty subtree_set
proof fn test_new_container_empty_subtree(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().subtree_set@ =~= Set::<ContainerPtr>::empty());
}

// Test 10: Parent's children list grew by 1
proof fn test_parent_children_grew(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[container_ptr].value().children@
        =~= old_container_perms[container_ptr].value().children@.push(new_container_ptr));
    assert(new_container_perms[container_ptr].value().children.len()
        == old_container_perms[container_ptr].value().children.len() + 1);
}

// Test 11: Parent's depth and uppertree unchanged
proof fn test_parent_preserved(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[container_ptr].value().depth =~= old_container_perms[container_ptr].value().depth);
    assert(new_container_perms[container_ptr].value().uppertree_seq =~= old_container_perms[container_ptr].value().uppertree_seq);
    assert(new_container_perms[container_ptr].value().parent =~= old_container_perms[container_ptr].value().parent);
    assert(new_container_perms[container_ptr].value().parent_rev_ptr =~= old_container_perms[container_ptr].value().parent_rev_ptr);
}

// Test 12: New container's uppertree_seq = parent's uppertree pushed with parent
proof fn test_new_container_uppertree(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().uppertree_seq@
        =~= old_container_perms[container_ptr].value().uppertree_seq@.push(container_ptr));
}

// Test 13: Both old and new subtree_set_wf hold
proof fn test_both_subtree_wf(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(container_subtree_set_wf(root_container, old_container_perms));
    new_container_preserve_tree_inv_4(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr);
    assert(container_subtree_set_wf(root_container, new_container_perms));
}

// Test 14: New container's parent_rev_ptr is set and matches children's node ref
proof fn test_parent_rev_ptr_match(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
    new_container_ptr: ContainerPtr,
)
    requires new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr),
{
    assert(new_container_perms[new_container_ptr].value().parent_rev_ptr.is_Some());
    assert(new_container_perms[container_ptr].value().children.get_node_ref(new_container_ptr)
        == new_container_perms[new_container_ptr].value().parent_rev_ptr.unwrap());
}

}
