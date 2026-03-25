use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type CpuId = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// syscall_new_container_with_endpoint or its helpers.
// All tests SHOULD FAIL verification.

// Test 1: syscall_new_container_with_endpoint requires thread_dom().contains(thread_ptr).
// Violating this by asserting membership when excluded.
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

// Test 2: endpoint_index must be in range [0, MAX_NUM_ENDPOINT_DESCRIPTORS).
// Using endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= endpoint_index && endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: The requirement checks quota.mem_4k >= 3 + init_quota.mem_4k.
// With container quota.mem_4k == 2 and init_quota.mem_4k == 0, this fails (2 < 3).
// SHOULD FAIL
proof fn test_boundary_insufficient_mem_4k_quota() {
    let container_mem_4k: usize = 2;
    let init_quota_mem_4k: usize = 0;
    assert(container_mem_4k >= 3 + init_quota_mem_4k);
}

// Test 4: The requirement checks depth != usize::MAX.
// With depth == usize::MAX, the new container cannot be created.
// SHOULD FAIL
proof fn test_boundary_depth_at_max() {
    let depth: usize = usize::MAX;
    assert(depth != usize::MAX);
}

// Test 5: The requirement checks children list is not full (len < CONTAINER_CHILD_LIST_LEN).
// With children.len() == CONTAINER_CHILD_LIST_LEN, the container is full.
// SHOULD FAIL
proof fn test_boundary_children_list_full() {
    let children_len: usize = CONTAINER_CHILD_LIST_LEN;
    assert(children_len < CONTAINER_CHILD_LIST_LEN);
}

// Test 6: Three distinct page pointers are required (page_ptr_1 != page_ptr_2 etc).
// If page_ptr_1 == page_ptr_2, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_duplicate_page_ptrs() {
    let page_ptr_1: PagePtr = 0x1000;
    let page_ptr_2: PagePtr = 0x1000;
    assert(page_ptr_1 != page_ptr_2);
}

// Test 7: The requirement checks that 3 + init_quota.mem_4k < usize::MAX (overflow guard).
// With init_quota.mem_4k very large, this cannot hold.
// SHOULD FAIL
proof fn test_boundary_mem_4k_overflow(
    init_quota_mem_4k: usize,
)
    requires
        init_quota_mem_4k == usize::MAX,
{
    assert(3 + init_quota_mem_4k < usize::MAX);
}

// Test 8: The requirement checks quota.pcid >= 1 + init_quota.pcid.
// With container pcid == 0, the container has no pcid quota to give.
// SHOULD FAIL
proof fn test_boundary_zero_pcid_quota() {
    let container_pcid: usize = 0;
    let init_quota_pcid: usize = 0;
    assert(container_pcid >= 1 + init_quota_pcid);
}

// Test 9: va_range.len * 3 < usize::MAX is required.
// With large va_range.len, the multiplication overflows.
// SHOULD FAIL
proof fn test_boundary_va_range_len_overflow(
    va_range_len: usize,
)
    requires
        va_range_len > usize::MAX / 4,
{
    assert(va_range_len * 3 < usize::MAX);
}

// Test 10: init_quota.mem_4k >= 3 * va_range.len is required.
// With init_quota.mem_4k == 0 and va_range.len == 1, this fails.
// SHOULD FAIL
proof fn test_boundary_init_quota_less_than_3x_va_range() {
    let init_quota_mem_4k: usize = 0;
    let va_range_len: usize = 1;
    assert(init_quota_mem_4k >= 3 * va_range_len);
}

}
