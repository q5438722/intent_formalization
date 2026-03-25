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

	#[verifier::external_body]
    pub proof fn unique_implys_no_duplicates(&self)
        requires
            self.unique(),
            self.wf(),
        ensures
            self@.no_duplicates(),
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
pub proof fn no_child_imply_no_subtree(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    c_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(c_ptr),
        container_perms[c_ptr].value().children@ =~= Seq::empty(),
    ensures
        container_perms[c_ptr].value().subtree_set@ =~= Set::empty(),
	{
		unimplemented!()
	}

pub open spec fn remove_container_ensures(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
) -> bool {
    &&& container_perms_wf(old_container_perms)
    &&& container_perms_wf(new_container_perms)
    &&& container_tree_wf(root_container, old_container_perms)
    &&& old_container_perms.dom().contains(container_ptr)
    &&& container_ptr != root_container
    &&& old_container_perms[container_ptr].value().children@ == Seq::<ContainerPtr>::empty()
    &&& old_container_perms[container_ptr].value().parent.is_Some()
    &&& old_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children@.remove_value(container_ptr) ==
            new_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children@
    &&& new_container_perms.dom() == old_container_perms.dom().remove(container_ptr)
    &&& forall|c_ptr: ContainerPtr|
            #![trigger old_container_perms.dom().contains(c_ptr)]
            old_container_perms.dom().contains(c_ptr) && c_ptr != container_ptr
                ==> new_container_perms[c_ptr].value().parent
                =~= old_container_perms[c_ptr].value().parent
                && new_container_perms[c_ptr].value().parent_rev_ptr
                =~= old_container_perms[c_ptr].value().parent_rev_ptr
                && (c_ptr != old_container_perms[container_ptr].value().parent.unwrap() ==>
                new_container_perms[c_ptr].value().children
                =~= old_container_perms[c_ptr].value().children)
                && new_container_perms[c_ptr].value().depth =~= old_container_perms[c_ptr].value().depth
                && new_container_perms[c_ptr].value().uppertree_seq
                =~= old_container_perms[c_ptr].value().uppertree_seq
    &&& forall|c_ptr: ContainerPtr|
            #![trigger old_container_perms[container_ptr].value().uppertree_seq@.contains(c_ptr)]
            old_container_perms[container_ptr].value().uppertree_seq@.contains(c_ptr)
                ==> new_container_perms[c_ptr].value().subtree_set@
                =~= old_container_perms[c_ptr].value().subtree_set@.remove(container_ptr)
    &&& forall|c_ptr: ContainerPtr|
            #![trigger old_container_perms.dom().contains(c_ptr)]
            new_container_perms.dom().contains(c_ptr)
                && old_container_perms[container_ptr].value().uppertree_seq@.contains(c_ptr) == false 
                ==> new_container_perms[c_ptr].value().subtree_set
                    =~= old_container_perms[c_ptr].value().subtree_set
    &&& forall|v:ContainerPtr|
            #![auto]
            new_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children@.contains(v) ==> 
                old_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children.get_node_ref(v) == 
                    new_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children.get_node_ref(v)
}

pub proof fn remove_container_preserve_tree_inv_6(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
)
    requires
        remove_container_ensures(
            root_container,
            old_container_perms,
            new_container_perms,
            container_ptr,
        ),
    ensures
        container_uppertree_seq_wf(
            root_container,
            new_container_perms,
        ),
{
    seq_remove_lemma::<ContainerPtr>();
    seq_remove_lemma_2::<ContainerPtr>();
    old_container_perms[old_container_perms[container_ptr].value().parent.unwrap()].value().children.unique_implys_no_duplicates();
    no_child_imply_no_subtree(
        root_container,
        old_container_perms,
        container_ptr,
    );
}


	#[verifier::external_body]
pub proof fn seq_remove_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] != v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ).contains(v),
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.subrange(0,i), s.contains(v)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ).contains(v) == false,
        forall|s: Seq<A>, i: int, j: int|
            #![trigger s.subrange(0,i), s[j]]
            0 <= i < s.len() && 0 <= j < i ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j] == s[j],
        forall|s: Seq<A>, i: int, j: int|
            #![trigger s.subrange(0,i), s[j+1]]
            0 <= i < s.len() && i <= j < s.len() - 1 ==> s.subrange(0, i).add(s.subrange(i + 1, s.len() as int))[j]
                == s[j + 1],
        forall|s: Seq<A>, v: A, i: int|
            #![trigger s.remove_value(v), s.subrange(0,i)]
            0 <= i < s.len() && s.contains(v) && s[i] == v && s.no_duplicates() ==> s.subrange(0, i).add(
                s.subrange(i + 1, s.len() as int),
            ) == s.remove_value(v),
	{
		unimplemented!()
	}

	#[verifier::external_body]
pub proof fn seq_remove_lemma_2<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A|
            x != v && s.no_duplicates() ==> s.remove_value(x).contains(v) == s.contains(v),
        forall|s: Seq<A>, v: A|
            #![auto]
            s.no_duplicates() ==> s.remove_value(v).contains(v) == false,
	{
		unimplemented!()
	}


// ============================================================
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but assert incorrect
// output relations to check if the spec rejects them.
// ============================================================

// SHOULD FAIL
// Test: Assert the removed container is still in the new domain.
// The spec says new_container_perms.dom() == old_container_perms.dom().remove(container_ptr),
// so container_ptr should NOT be in the new domain.
proof fn test_mutation_removed_still_in_domain(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
)
    requires
        remove_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr),
{
    assert(new_container_perms.dom().contains(container_ptr));
}

// SHOULD FAIL
// Test: Assert that the full container_tree_wf holds for new_container_perms.
// The function only ensures container_uppertree_seq_wf, not the full tree invariant.
proof fn test_mutation_full_tree_wf(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
)
    requires
        remove_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr),
{
    remove_container_preserve_tree_inv_6(root_container, old_container_perms, new_container_perms, container_ptr);
    assert(container_tree_wf(root_container, new_container_perms));
}

// SHOULD FAIL
// Test: Assert that the parent's children sequence is unchanged after removal.
// The spec says children are updated to remove container_ptr.
proof fn test_mutation_parent_children_same(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
)
    requires
        remove_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr),
{
    let parent_ptr = old_container_perms[container_ptr].value().parent.unwrap();
    // The parent's children should have container_ptr removed — asserting same should fail.
    assert(new_container_perms[parent_ptr].value().children@ =~= old_container_perms[parent_ptr].value().children@);
}

// SHOULD FAIL
// Test: Assert that the domain of new perms equals the domain of old perms (no removal).
// The spec says dom(new) = dom(old).remove(container_ptr).
proof fn test_mutation_domain_unchanged(
    root_container: ContainerPtr,
    old_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    new_container_perms: Map<ContainerPtr, PointsTo<Container>>,
    container_ptr: ContainerPtr,
)
    requires
        remove_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr),
{
    assert(new_container_perms.dom() =~= old_container_perms.dom());
}


}
