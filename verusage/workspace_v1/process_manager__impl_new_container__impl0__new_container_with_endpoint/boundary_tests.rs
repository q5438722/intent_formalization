use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal type/const definitions from target file
// ============================================================

pub type IOid = usize;
pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
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
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;
pub const PAGE_SZ_2m: usize = 1usize << 21;
pub const PAGE_SZ_1g: usize = 1usize << 30;

// ============================================================
// Quota definition
// ============================================================
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

// ===================== BOUNDARY TESTS =====================
// Each test encodes a property that violates a precondition (requires)
// of new_container_with_endpoint. These are intended to FAIL verification.

// ============================================================
// BOUNDARY TEST 1: thread_ptr NOT in thread_dom
// Precondition: old(self).thread_dom().contains(thread_ptr)
// Using a thread_ptr not in the domain violates this.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: endpoint_index out of range (>= MAX_NUM_ENDPOINT_DESCRIPTORS)
// Precondition: 0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
// Using endpoint_index == 128 violates the upper bound.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_endpoint_index_out_of_range()
{
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // 128
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: container depth == usize::MAX
// Precondition: get_container(get_thread(thread_ptr).owning_container).depth != usize::MAX
// When depth is usize::MAX, adding 1 would overflow.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_container_depth_max()
{
    let depth: usize = usize::MAX;
    assert(depth != usize::MAX); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: children list at full capacity
// Precondition: container.children.len() < CONTAINER_CHILD_LIST_LEN
// When children.len() == CONTAINER_CHILD_LIST_LEN, list is full.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_children_list_full()
{
    let children_len: usize = CONTAINER_CHILD_LIST_LEN; // 10
    assert(children_len < CONTAINER_CHILD_LIST_LEN); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: insufficient mem_4k quota (parent has only 2, needs >= 3)
// Precondition: quota.mem_4k - 3 >= new_quota.mem_4k
// With parent.mem_4k == 2 and new_quota.mem_4k == 0, 2 - 3 < 0.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_insufficient_mem_4k_quota()
{
    let parent_mem_4k: usize = 2;
    let new_quota_mem_4k: usize = 0;
    assert(parent_mem_4k >= 3 + new_quota_mem_4k); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: page pointers are not distinct (page_ptr_1 == page_ptr_2)
// Precondition: page_ptr_1 != page_ptr_2
// SHOULD FAIL
// ============================================================
proof fn test_boundary_page_ptrs_not_distinct()
{
    let page_ptr_1: PagePtr = 0x1000;
    let page_ptr_2: PagePtr = 0x1000; // same as page_ptr_1
    assert(page_ptr_1 != page_ptr_2); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: page_ptr already in page_closure
// Precondition: old(self).page_closure().contains(page_ptr_1) == false
// If page_ptr_1 is already in page_closure, the precondition fails.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_page_ptr_already_in_closure()
{
    let page_closure: Set<PagePtr> = Set::empty().insert(0x1000).insert(0x2000);
    let page_ptr_1: PagePtr = 0x1000; // already in closure
    assert(page_closure.contains(page_ptr_1) == false); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 8: spec_greater violated (new_quota.mem_2m > parent.mem_2m)
// Precondition: parent.quota.spec_greater(new_quota)
// This requires parent.mem_2m >= new_quota.mem_2m, etc.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_quota_not_greater()
{
    let parent_quota = Quota { mem_4k: 100, mem_2m: 5, mem_1g: 1, pcid: 10, ioid: 5 };
    let new_quota = Quota { mem_4k: 10, mem_2m: 10, mem_1g: 0, pcid: 1, ioid: 0 };
    // parent.mem_2m (5) < new_quota.mem_2m (10), violating spec_greater
    assert(parent_quota.spec_greater(&new_quota)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 9: new_pcid matches an existing process's pcid
// Precondition: forall p_ptr in proc_dom: get_proc(p_ptr).pcid != new_pcid
// If new_pcid collides with an existing pcid, the precondition fails.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_pcid_collision()
{
    let existing_pcid: Pcid = 42;
    let new_pcid: Pcid = 42; // same as existing
    assert(existing_pcid != new_pcid); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 10: page_ptr_2 == page_ptr_3 (third pair not distinct)
// Precondition: page_ptr_2 != page_ptr_3
// SHOULD FAIL
// ============================================================
proof fn test_boundary_page_ptr_2_equals_3()
{
    let page_ptr_2: PagePtr = 0x3000;
    let page_ptr_3: PagePtr = 0x3000; // same as page_ptr_2
    assert(page_ptr_2 != page_ptr_3); // SHOULD FAIL
}

} // verus!
