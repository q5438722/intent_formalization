use vstd::prelude::*;

fn main() {}

verus!{

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const PCID_MAX: usize = 4096;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_mmap to check if invalid inputs are properly
// rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: syscall_mmap requires thread_ptr in thread_dom.
// If thread_ptr is NOT in thread_dom, precondition is violated.
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

// Test 2: syscall_mmap requires va_range.len * 4 < usize::MAX.
// With a sufficiently large len, this precondition is violated.
// SHOULD FAIL
proof fn test_boundary_va_range_len_overflow(len: usize)
    requires
        len * 4 >= usize::MAX,
{
    // This contradicts the precondition of syscall_mmap
    assert(len * 4 < usize::MAX);
}

// Test 3: va_range.wf() requires start + len * 4096 < usize::MAX.
// With a very large start value, the sum overflows.
// SHOULD FAIL
proof fn test_boundary_va_range_start_overflow(start: usize, len: usize)
    requires
        start > usize::MAX - 4096,
        len == 1,
{
    // start + len * 4096 overflows usize::MAX
    assert(start + len * 4096 < usize::MAX);
}

// Test 4: va_range.wf() requires len matches the view length.
// With va_range.len == 0, the range is empty — edge case.
// The spec checks container quota >= 0*4=0, which always passes,
// but we test that len == 0 still means no pages allocated.
// With quota < 0 (impossible for usize), we violate the check.
// SHOULD FAIL
proof fn test_boundary_zero_len_va_range_with_zero_quota() {
    let quota_mem_4k: usize = 0;
    let va_range_len: usize = 1;
    // quota < va_range_len * 4 means ErrorNoQuota
    // Mutated: claim quota >= va_range_len * 4
    assert(quota_mem_4k >= va_range_len * 4);
}

// Test 5: page_ptr2page_index requires ptr % 0x1000 == 0.
// An unaligned pointer violates this.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    let ptr: usize = 0x1001;
    assert(ptr % 0x1000 == 0);
}

// Test 6: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Index at NUM_PAGES is out of bounds.
// SHOULD FAIL
proof fn test_boundary_page_index_at_num_pages() {
    let i: usize = NUM_PAGES;
    assert(i < NUM_PAGES);
}

// Test 7: The quota check requires mem_4k >= va_range.len * 4.
// At exact boundary mem_4k == va_range.len * 4 - 1, the check fails.
// SHOULD FAIL
proof fn test_boundary_quota_exactly_insufficient(
    va_range_len: usize,
    quota_mem_4k: usize,
)
    requires
        va_range_len == 10,
        quota_mem_4k < va_range_len * 4,
{
    // quota_mem_4k < va_range_len * 4, so ErrorNoQuota
    // Claim: quota is sufficient
    assert(quota_mem_4k >= va_range_len * 4);
}

// Test 8: va_range addresses must be 4k-valid.
// A VA that is not 4k-aligned should not pass wf().
// SHOULD FAIL
proof fn test_boundary_non_4k_aligned_va() {
    let va: usize = 0x123; // not 4k aligned
    // va & (!MEM_4k_MASK) should be 0 for validity
    // 0x123 & (!0x0000_ffff_ffff_f000) != 0
    let mem_4k_mask: u64 = 0x0000_ffff_ffff_f000u64;
    assert(va & (!mem_4k_mask) as usize == 0);
}

// Test 9: va_range must have no duplicate addresses in its view.
// If two indices map to the same VA, no_duplicates() is violated.
// SHOULD FAIL
proof fn test_boundary_duplicate_va_in_range(
    va_seq: Seq<VAddr>,
)
    requires
        va_seq.len() >= 2,
        va_seq[0] == va_seq[1],
{
    assert(va_seq.no_duplicates());
}

// Test 10: syscall_mmap requires old(self).total_wf().
// total_wf requires total_mem_4k_quota_wf, meaning free pages
// equals sum of all container quotas. With zero free pages and
// positive quota requirement, the total_wf invariant is broken.
// SHOULD FAIL
proof fn test_boundary_free_pages_less_than_quota() {
    let free_pages: usize = 0;
    let total_quota: usize = 100;
    // total_wf requires: free_pages == total_quota (via fold)
    assert(free_pages == total_quota);
}

}
