use vstd::prelude::*;

fn main() {}

verus!{

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type IOid = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_io_mmap and related functions to check if
// invalid inputs are properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: thread_ptr must be in thread_dom.
// Violating this precondition should be rejected.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    // Attempt to assert thread is in domain despite precondition violation
    assert(thread_dom.contains(thread_ptr));
}

// Test 2: va_range.len * 4 must be < usize::MAX.
// At the exact boundary (usize::MAX / 4), multiplication overflows.
// SHOULD FAIL
proof fn test_boundary_va_range_overflow() {
    let len: usize = (usize::MAX / 4 + 1) as usize;
    assert(len * 4 < usize::MAX);
}

// Test 3: spec_va_4k_valid requires proper alignment and range.
// A pointer with low bits set violates 4K alignment.
// SHOULD FAIL
proof fn test_boundary_invalid_va_alignment() {
    let va: usize = 0x1001usize; // not 4K-aligned
    assert(va & (!MEM_4k_MASK) as usize == 0);
}

// Test 4: page_ptr2page_index requires ptr % 0x1000 == 0.
// An unaligned pointer violates this.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    let ptr: usize = 0x1001usize;
    assert(ptr % 0x1000 == 0);
}

// Test 5: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// An index at NUM_PAGES violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_page_index_at_num_pages() {
    let i: usize = NUM_PAGES;
    assert(i < NUM_PAGES);
}

// Test 6: Container quota mem_4k must be >= va_range.len * 4.
// With quota = 0 and len > 0, this is violated.
// SHOULD FAIL
proof fn test_boundary_zero_quota_nonzero_len() {
    let quota_mem_4k: usize = 0;
    let va_range_len: usize = 1;
    assert(quota_mem_4k >= va_range_len * 4);
}

// Test 7: Container quota exactly at boundary: quota = len*4 - 1.
// This is strictly less than required.
// SHOULD FAIL
proof fn test_boundary_quota_off_by_one() {
    let va_range_len: usize = 10;
    let quota_mem_4k: usize = (va_range_len * 4 - 1) as usize;
    assert(quota_mem_4k >= va_range_len * 4);
}

// Test 8: Free pages must be >= va_range.len * 4.
// With zero free pages and nonzero len, this fails.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    let va_range_len: usize = 1;
    assert(free_pages >= va_range_len * 4);
}

// Test 9: page_index2page_ptr with usize::MAX is far beyond range.
// SHOULD FAIL
proof fn test_boundary_page_index_usize_max() {
    let i: usize = usize::MAX;
    assert(i < NUM_PAGES);
}

// Test 10: Multiple precondition violations simultaneously.
// Thread not in domain AND va_range overflow AND zero quota.
// SHOULD FAIL
proof fn test_boundary_multiple_violations(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    let va_range_len: usize = usize::MAX;
    let quota_mem_4k: usize = 0;
    assert(
        thread_dom.contains(thread_ptr)
        && va_range_len * 4 < usize::MAX
        && quota_mem_4k >= va_range_len * 4
    );
}

}
