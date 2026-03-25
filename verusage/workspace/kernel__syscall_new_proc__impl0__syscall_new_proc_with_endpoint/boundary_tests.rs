use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// syscall_new_proc_with_endpoint or its callees.
// All tests SHOULD FAIL verification.

// Test 1: syscall_new_proc_with_endpoint requires thread_dom().contains(thread_ptr).
// If the thread is not in the domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// Test 2: syscall_new_proc_with_endpoint requires 0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS.
// endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS is out of range.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= endpoint_index && endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: syscall_new_proc_with_endpoint requires va_range.len * 3 + 3 < usize::MAX.
// With va_range.len near usize::MAX / 3, overflow occurs.
// SHOULD FAIL
proof fn test_boundary_va_range_len_overflow() {
    let len: usize = usize::MAX / 3;
    assert(len * 3 + 3 < usize::MAX);
}

// Test 4: requirement checks get_container_quota(container_ptr).mem_4k >= va_range.len * 3 + 2.
// With quota.mem_4k == 0 and va_range.len == 1, requirement fails.
// SHOULD FAIL
proof fn test_boundary_zero_quota_for_new_proc() {
    let mem_4k: usize = 0;
    let va_range_len: usize = 1;
    assert(mem_4k >= va_range_len * 3 + 2);
}

// Test 5: requirement checks get_num_of_free_pages() >= va_range.len * 3 + 2.
// With 0 free pages, cannot allocate.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    let va_range_len: usize = 1;
    assert(free_pages >= va_range_len * 3 + 2);
}

// Test 6: requirement checks children.len() < PROC_CHILD_LIST_LEN.
// If children list is full (== PROC_CHILD_LIST_LEN), requirement fails.
// SHOULD FAIL
proof fn test_boundary_children_list_full() {
    let children_len: usize = PROC_CHILD_LIST_LEN;
    assert(children_len < PROC_CHILD_LIST_LEN);
}

// Test 7: requirement checks depth != usize::MAX.
// If depth == usize::MAX, the requirement is violated.
// SHOULD FAIL
proof fn test_boundary_depth_at_max() {
    let depth: usize = usize::MAX;
    assert(depth != usize::MAX);
}

// Test 8: requirement checks scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN.
// If scheduler is full, requirement fails.
// SHOULD FAIL
proof fn test_boundary_scheduler_full() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 9: requirement checks owned_procs.len() < CONTAINER_PROC_LIST_LEN.
// If proc list is full, requirement fails.
// SHOULD FAIL
proof fn test_boundary_proc_list_full() {
    let owned_procs_len: usize = CONTAINER_PROC_LIST_LEN;
    assert(owned_procs_len < CONTAINER_PROC_LIST_LEN);
}

// Test 10: requirement checks get_is_pcid_exhausted() == false,
// i.e., free_pcids.len() > 0. With 0 free pcids, should fail.
// SHOULD FAIL
proof fn test_boundary_pcid_exhausted() {
    let free_pcids_len: usize = 0;
    assert(free_pcids_len > 0);
}

// Test 11: endpoint_shareable requires endpoint_ptr_by_idx.is_Some().
// If None, the endpoint is not shareable.
// SHOULD FAIL
proof fn test_boundary_endpoint_ptr_is_none() {
    let endpoint_ptr: Option<EndpointPtr> = None;
    assert(endpoint_ptr.is_Some());
}

// Test 12: endpoint_shareable requires rf_counter != usize::MAX.
// If rf_counter == usize::MAX, endpoint is not shareable.
// SHOULD FAIL
proof fn test_boundary_rf_counter_at_max() {
    let rf_counter: usize = usize::MAX;
    assert(rf_counter != usize::MAX);
}

// Test 13: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 violates alignment precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 14: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i == NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_num_pages() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 15: va_range.wf() requires va_range elements are valid.
// VA == 0 is in kernel space and not valid for user mapping.
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

}
