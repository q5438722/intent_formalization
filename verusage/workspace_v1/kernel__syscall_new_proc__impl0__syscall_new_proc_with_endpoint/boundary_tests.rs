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
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// ============================================================
// BOUNDARY TEST 1: thread_ptr NOT in thread_dom
// The precondition requires old(self).thread_dom().contains(thread_ptr).
// Using a thread_ptr not in the domain violates the precondition.
// SHOULD FAIL: thread_ptr not contained in thread_dom
// ============================================================
proof fn test_boundary_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: endpoint_index out of range (>= MAX_NUM_ENDPOINT_DESCRIPTORS)
// The precondition requires 0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS (128).
// Using endpoint_index == 128 violates the precondition.
// SHOULD FAIL: endpoint_index at upper bound is out of range
// ============================================================
proof fn test_boundary_endpoint_index_out_of_range()
{
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // == 128, out of range
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: quota.mem_4k < 2 (insufficient quota)
// The syscall_new_proc_with_endpoint_requirement checks
// quota.mem_4k < va_range.len * 3 + 2 => false.
// With va_range.len == 0, we need mem_4k >= 2.
// Setting mem_4k == 1 should cause the requirement to fail.
// SHOULD FAIL: quota too low
// ============================================================
proof fn test_boundary_quota_insufficient()
{
    let mem_4k: usize = 1;
    let va_range_len: usize = 0;
    // va_range_len * 3 + 2 == 2, but mem_4k == 1 < 2
    assert(mem_4k >= (va_range_len * 3 + 2) as usize); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: process children list at capacity
// The requirement checks get_is_process_childern_list_full(proc_ptr),
// which is children.len() >= PROC_CHILD_LIST_LEN (10).
// When children.len() == 10, the requirement returns false.
// SHOULD FAIL: children list is full
// ============================================================
proof fn test_boundary_proc_children_full()
{
    let children_count: usize = PROC_CHILD_LIST_LEN; // == 10
    assert(children_count < PROC_CHILD_LIST_LEN); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: scheduler at capacity
// The requirement checks get_is_scheduler_full(container_ptr),
// which is scheduler.len() >= MAX_CONTAINER_SCHEDULER_LEN (10).
// When scheduler.len() == 10, the requirement returns false.
// SHOULD FAIL: scheduler is full
// ============================================================
proof fn test_boundary_scheduler_full()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN; // == 10
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: container proc list at capacity
// The requirement checks get_is_proc_list_full(container_ptr),
// which is owned_procs.len() >= CONTAINER_PROC_LIST_LEN (10).
// When owned_procs.len() == 10, the requirement returns false.
// SHOULD FAIL: container proc list is full
// ============================================================
proof fn test_boundary_container_proc_list_full()
{
    let proc_list_len: usize = CONTAINER_PROC_LIST_LEN; // == 10
    assert(proc_list_len < CONTAINER_PROC_LIST_LEN); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: depth overflow (depth == usize::MAX)
// The requirement checks get_is_process_depth_overflow(proc_ptr),
// which is depth >= usize::MAX.
// When depth == usize::MAX, the requirement returns false.
// SHOULD FAIL: depth is at maximum
// ============================================================
proof fn test_boundary_depth_overflow()
{
    let depth: usize = usize::MAX;
    assert(depth < usize::MAX); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 8: page_ptr_1 == page_ptr_2 (must be distinct)
// The precondition requires page_ptr_1 != page_ptr_2.
// Using the same value violates the precondition.
// SHOULD FAIL: page pointers must be distinct
// ============================================================
proof fn test_boundary_page_ptrs_equal()
{
    let page_ptr_1: PagePtr = 42;
    let page_ptr_2: PagePtr = 42;
    assert(page_ptr_1 != page_ptr_2); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 9: pcid not unique (new_pcid already in use)
// The precondition requires forall p_ptr in proc_dom:
//   get_proc(p_ptr).pcid != new_pcid.
// If any existing proc has the same pcid, it violates the precondition.
// SHOULD FAIL: new_pcid collides with existing pcid
// ============================================================
proof fn test_boundary_pcid_collision()
{
    let existing_pcid: Pcid = 7;
    let new_pcid: Pcid = 7;
    assert(existing_pcid != new_pcid); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 10: page_ptr already in page_closure
// The precondition requires page_ptr_1 NOT in page_closure.
// If it is already present, the precondition is violated.
// SHOULD FAIL: page_ptr already in closure
// ============================================================
proof fn test_boundary_page_ptr_in_closure()
{
    let page_closure: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let page_ptr_1: PagePtr = 100;
    assert(page_closure.contains(page_ptr_1) == false); // SHOULD FAIL
}

} // verus!
