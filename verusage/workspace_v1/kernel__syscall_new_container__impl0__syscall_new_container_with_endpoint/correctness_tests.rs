use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Combined correctness tests for syscall_new_container_with_endpoint
// ============================================================

pub type IOid = usize;
pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const CONTAINER_ENDPOINT_LIST_LEN: usize = 10;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_greater(&self, new: &Quota) -> bool {
        &&& self.mem_4k >= new.mem_4k
        &&& self.mem_2m >= new.mem_2m
        &&& self.mem_1g >= new.mem_1g
        &&& self.pcid >= new.pcid
        &&& self.ioid >= new.ioid
    }
}

// ==================== BOUNDARY TESTS ====================

// SHOULD FAIL: thread_ptr not in domain
proof fn test_boundary_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// SHOULD FAIL: endpoint_index >= MAX_NUM_ENDPOINT_DESCRIPTORS
proof fn test_boundary_endpoint_index_out_of_range()
{
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// SHOULD FAIL: container depth == usize::MAX
proof fn test_boundary_container_depth_max()
{
    let depth: usize = usize::MAX;
    assert(depth != usize::MAX); // SHOULD FAIL
}

// SHOULD FAIL: children list full
proof fn test_boundary_children_list_full()
{
    let children_len: usize = CONTAINER_CHILD_LIST_LEN;
    assert(!(children_len >= CONTAINER_CHILD_LIST_LEN)); // SHOULD FAIL
}

// SHOULD FAIL: insufficient mem_4k quota
proof fn test_boundary_insufficient_mem_4k_quota()
{
    let parent_mem_4k: usize = 2;
    let init_mem_4k: usize = 0;
    assert(parent_mem_4k >= 3 + init_mem_4k); // SHOULD FAIL
}

// SHOULD FAIL: insufficient pcid quota
proof fn test_boundary_insufficient_pcid_quota()
{
    let parent_pcid: usize = 0;
    let init_pcid: usize = 0;
    assert(parent_pcid >= 1 + init_pcid); // SHOULD FAIL
}

// SHOULD FAIL: zero free pages
proof fn test_boundary_no_free_pages()
{
    let free_pages: usize = 0;
    let init_mem_4k: usize = 0;
    assert(free_pages >= 3 + init_mem_4k); // SHOULD FAIL
}

// SHOULD FAIL: page pointers not distinct
proof fn test_boundary_page_ptrs_not_distinct()
{
    let page_ptr_1: PagePtr = 0x1000;
    let page_ptr_2: PagePtr = 0x1000;
    assert(page_ptr_1 != page_ptr_2); // SHOULD FAIL
}

// SHOULD FAIL: init_quota.mem_4k < 3 * va_range.len
proof fn test_boundary_init_quota_less_than_va_range_cost()
{
    let init_mem_4k: usize = 2;
    let va_range_len: usize = 1;
    assert(init_mem_4k >= 3 * va_range_len); // SHOULD FAIL
}

// SHOULD FAIL: pcid exhausted
proof fn test_boundary_pcid_exhausted()
{
    let free_pcids_len: usize = 0;
    assert(free_pcids_len > 0); // SHOULD FAIL
}

// ==================== BEHAVIORAL MUTATION TESTS ====================

// SHOULD FAIL: new container children non-empty
proof fn test_mutation_new_container_children_not_empty()
{
    let children: Seq<ContainerPtr> = Seq::empty();
    assert(children.len() > 0); // SHOULD FAIL
}

// SHOULD FAIL: owned_procs is empty (missing process)
proof fn test_mutation_new_container_no_owned_proc()
{
    let page_ptr_2: ProcPtr = 0x2000;
    let owned_procs: Seq<ProcPtr> = Seq::empty().push(page_ptr_2);
    assert(owned_procs =~= Seq::<ProcPtr>::empty()); // SHOULD FAIL
}

// SHOULD FAIL: wrong owning container for new process
proof fn test_mutation_proc_wrong_owning_container()
{
    let page_ptr_1: ContainerPtr = 0x1000;
    let page_ptr_2: ContainerPtr = 0x2000;
    let new_proc_owning_container: ContainerPtr = page_ptr_1;
    assert(new_proc_owning_container == page_ptr_2); // SHOULD FAIL
}

// SHOULD FAIL: parent children not updated
proof fn test_mutation_parent_children_not_updated()
{
    let old_children: Seq<ContainerPtr> = Seq::empty().push(100).push(200);
    let page_ptr_1: ContainerPtr = 300;
    let new_children: Seq<ContainerPtr> = old_children.push(page_ptr_1);
    assert(new_children =~= old_children); // SHOULD FAIL
}

// SHOULD FAIL: thread wrong container
proof fn test_mutation_thread_wrong_owning_container()
{
    let page_ptr_1: ContainerPtr = 0x1000;
    let parent_container: ContainerPtr = 0x5000;
    let thread_owning_container: ContainerPtr = page_ptr_1;
    assert(thread_owning_container == parent_container); // SHOULD FAIL
}

// SHOULD FAIL: process wrong pcid
proof fn test_mutation_proc_wrong_pcid()
{
    let new_pcid: Pcid = 42;
    let proc_pcid: Pcid = new_pcid;
    assert(proc_pcid == 43); // SHOULD FAIL
}

// SHOULD FAIL: wrong quota
proof fn test_mutation_new_container_wrong_quota()
{
    let init_quota = Quota { mem_4k: 100, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    let assigned_quota = Quota { mem_4k: 100, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    assert(assigned_quota.mem_4k == 200); // SHOULD FAIL
}

// SHOULD FAIL: wrong mem_4k deduction (2 instead of 3)
proof fn test_mutation_wrong_mem_4k_deduction()
{
    let old_mem_4k: int = 100;
    let init_mem_4k: int = 10;
    let new_parent_mem_4k: int = old_mem_4k - 3 - init_mem_4k;
    assert(new_parent_mem_4k == old_mem_4k - 2 - init_mem_4k); // SHOULD FAIL
}

// SHOULD FAIL: new process's ioid is Some
proof fn test_mutation_proc_ioid_not_none()
{
    let ioid: Option<IOid> = None;
    assert(ioid is Some); // SHOULD FAIL
}

// SHOULD FAIL: endpoint domain changed
proof fn test_mutation_endpoint_dom_changed()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(1).insert(2);
    let new_endpoint_dom: Set<EndpointPtr> = old_endpoint_dom;
    let extra_endpoint: EndpointPtr = 999;
    assert(new_endpoint_dom.contains(extra_endpoint)); // SHOULD FAIL
}

// ==================== LOGICAL TESTS ====================

// SHOULD FAIL: allocation is deterministic
proof fn test_logical_determinism_of_allocation()
{
    let page_ptr_1a: PagePtr = 0x1000;
    let page_ptr_1b: PagePtr = 0x2000;
    assert(page_ptr_1a == page_ptr_1b); // SHOULD FAIL
}

// SHOULD FAIL: depth bounded by small constant
proof fn test_logical_depth_bounded_small()
{
    let parent_depth: int = 100;
    let new_depth: int = parent_depth + 1;
    assert(new_depth <= 10); // SHOULD FAIL
}

// SHOULD FAIL: new container already in subtree
proof fn test_logical_container_already_in_subtree()
{
    let parent_subtree: Set<ContainerPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_1: ContainerPtr = 300;
    assert(parent_subtree.contains(page_ptr_1)); // SHOULD FAIL
}

// SHOULD FAIL: container_dom and proc_dom disjoint
proof fn test_logical_container_proc_dom_disjoint()
{
    let container_dom: Set<ContainerPtr> = Set::empty().insert(1).insert(2).insert(3);
    let proc_dom: Set<ProcPtr> = Set::empty().insert(3).insert(4).insert(5);
    assert(container_dom.disjoint(proc_dom)); // SHOULD FAIL
}

// SHOULD FAIL: parent container unchanged
proof fn test_logical_parent_container_unchanged()
{
    let old_children: Seq<ContainerPtr> = Seq::empty().push(100);
    let page_ptr_1: ContainerPtr = 200;
    let new_children: Seq<ContainerPtr> = old_children.push(page_ptr_1);
    assert(new_children =~= old_children); // SHOULD FAIL
}

// SHOULD FAIL: new thread in old domain
proof fn test_logical_new_thread_in_old_dom()
{
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_3: ThreadPtr = 30;
    assert(old_thread_dom.contains(page_ptr_3)); // SHOULD FAIL
}

// SHOULD FAIL: new process has > 1 thread
proof fn test_logical_new_proc_multiple_threads()
{
    let page_ptr_3: ThreadPtr = 0x3000;
    let owned_threads: Seq<ThreadPtr> = Seq::empty().push(page_ptr_3);
    assert(owned_threads.len() > 1); // SHOULD FAIL
}

// SHOULD FAIL: mem_2m deduction includes extra 3
proof fn test_logical_mem_2m_extra_deduction()
{
    let old_mem_2m: int = 50;
    let init_mem_2m: int = 10;
    let new_parent_mem_2m: int = old_mem_2m - init_mem_2m;
    assert(new_parent_mem_2m == old_mem_2m - 3 - init_mem_2m); // SHOULD FAIL
}

// SHOULD FAIL: all endpoint descriptors filled
proof fn test_logical_all_endpoint_descriptors_filled()
{
    let endpoint_ptr: EndpointPtr = 42;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    assert(descriptors[1] is Some); // SHOULD FAIL
}

// SHOULD FAIL: new container has CPUs
proof fn test_logical_new_container_has_cpus()
{
    let owned_cpus: Set<CpuId> = Set::empty();
    assert(owned_cpus.len() > 0); // SHOULD FAIL
}

} // verus!
