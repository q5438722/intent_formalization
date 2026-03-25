use vstd::prelude::*;

fn main() {}

verus!{

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

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_ptr2page_index))]
pub fn page_ptr2page_index(ptr: usize) -> (ret: usize)
    requires
        ptr % 0x1000 == 0,
    ensures
        ret == spec_page_ptr2page_index(ptr),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires
        0 <= i < NUM_PAGES,
    ensures
        ret == spec_page_index2page_ptr(i),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_4k_valid(va),
{
    unimplemented!()
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or uses
// edge-case values. All tests SHOULD FAIL verification.

// Test 1: page_ptr2page_index requires ptr % 0x1000 == 0.
// Call with an unaligned pointer (1). Precondition violated.
// SHOULD FAIL
fn test_boundary_page_ptr2page_index_unaligned() {
    let ret = page_ptr2page_index(1);
}

// Test 2: page_ptr2page_index requires ptr % 0x1000 == 0.
// Call with ptr = 0x1001 (odd offset). Precondition violated.
// SHOULD FAIL
fn test_boundary_page_ptr2page_index_odd_offset() {
    let ret = page_ptr2page_index(0x1001);
}

// Test 3: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Call with i = NUM_PAGES (out of range). Precondition violated.
// SHOULD FAIL
fn test_boundary_page_index2page_ptr_at_max() {
    let ret = page_index2page_ptr(NUM_PAGES);
}

// Test 4: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Call with i = usize::MAX. Way out of range.
// SHOULD FAIL
fn test_boundary_page_index2page_ptr_usize_max() {
    let ret = page_index2page_ptr(usize::MAX);
}

// Test 5: syscall_io_mmap requires thread_dom().contains(thread_ptr).
// Assert that an arbitrary thread_ptr is in an empty domain.
// This simulates calling with a thread not in the domain.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(thread_ptr: ThreadPtr) {
    let thread_dom = Set::<ThreadPtr>::empty();
    assert(thread_dom.contains(thread_ptr));
}

// Test 6: syscall_io_mmap requires va_range.len * 4 < usize::MAX.
// With len = usize::MAX, len * 4 overflows.
// SHOULD FAIL
proof fn test_boundary_va_range_len_overflow() {
    let len: usize = usize::MAX;
    assert(len * 4 < usize::MAX);
}

// Test 7: check_io_space_va_range_free requires proc_dom().contains(target_proc_ptr).
// Assert that an arbitrary proc is in an empty domain.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_domain(proc_ptr: ProcPtr) {
    let proc_dom = Set::<ProcPtr>::empty();
    assert(proc_dom.contains(proc_ptr));
}

// Test 8: range_alloc_and_map_io requires quota.mem_4k >= 4 * va_range.len.
// With quota = 0 and len = 1, 0 >= 4 is false.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota() {
    let quota_mem_4k: usize = 0;
    let va_range_len: usize = 1;
    assert(quota_mem_4k >= 4 * va_range_len);
}

// Test 9: range_alloc_and_map_io requires get_num_of_free_pages() >= 4 * va_range.len.
// With 3 free pages and len = 1, 3 >= 4 is false.
// SHOULD FAIL
proof fn test_boundary_insufficient_free_pages() {
    let free_pages: usize = 3;
    let va_range_len: usize = 1;
    assert(free_pages >= 4 * va_range_len);
}

// Test 10: syscall_io_mmap requires va_range.wf(), which includes
// start + len * 4096 < usize::MAX. With start = usize::MAX - 1 and len = 1,
// the overflow check fails.
// SHOULD FAIL
proof fn test_boundary_va_range_overflow_start() {
    let start: usize = (usize::MAX - 1) as usize;
    let len: usize = 1;
    assert(start + len * 4096 < usize::MAX);
}

}
