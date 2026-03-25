use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
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

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or
// uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 (not 4k-aligned) violates the precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 2: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 3: page_index2page_ptr with i = usize::MAX overflows.
// SHOULD FAIL
proof fn test_boundary_page_index_overflow() {
    let i: usize = usize::MAX;
    assert(i < NUM_PAGES);
}

// Test 4: create_entry_and_alloc_and_map requires mem_4k >= 4.
// If mem_4k == 3, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota() {
    let mem_4k: usize = 3;
    assert(mem_4k >= 4);
}

// Test 5: create_entry_and_alloc_and_map requires
// get_num_of_free_pages() >= 4. Having only 3 free pages violates this.
// SHOULD FAIL
proof fn test_boundary_insufficient_free_pages() {
    let free_pages: usize = 3;
    assert(free_pages >= 4);
}

// Test 6: create_entry_and_alloc_and_map requires target_va is NOT
// already in the address space. If it IS in the domain, this is violated.
// SHOULD FAIL
proof fn test_boundary_va_already_mapped(
    addr_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        addr_space_dom.contains(target_va),
{
    assert(addr_space_dom.contains(target_va) == false);
}

// Test 7: range_alloc_and_map requires mem_4k >= 4 * va_range.len.
// If va_range.len = 10 and quota = 39, it's not enough (need 40).
// SHOULD FAIL
proof fn test_boundary_range_alloc_insufficient_quota() {
    let va_range_len: usize = 10;
    let mem_4k: usize = 39;
    assert(mem_4k >= 4 * va_range_len);
}

// Test 8: range_alloc_and_map requires get_num_of_free_pages >= 4 * len.
// If va_range.len = 5 and free = 19, it's not enough (need 20).
// SHOULD FAIL
proof fn test_boundary_range_alloc_insufficient_free_pages() {
    let va_range_len: usize = 5;
    let free_pages: usize = 19;
    assert(free_pages >= 4 * va_range_len);
}

// Test 9: range_alloc_and_map requires va_range.len * 4 < usize::MAX.
// A very large len would overflow.
// SHOULD FAIL
proof fn test_boundary_range_alloc_overflow() {
    let va_range_len: usize = usize::MAX / 2;
    assert(va_range_len * 4 < usize::MAX);
}

// Test 10: range_alloc_and_map requires ALL addresses in the range
// are NOT in the address space. If any single one is already mapped,
// the precondition fails.
// SHOULD FAIL
proof fn test_boundary_range_alloc_partial_overlap(
    addr_space: Map<VAddr, usize>,
    va: VAddr,
)
    requires
        addr_space.dom().contains(va),
{
    assert(addr_space.dom().contains(va) == false);
}

// Test 11: proc_dom must contain target_proc_ptr for both functions.
// If the proc is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    target_proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(target_proc_ptr),
{
    assert(proc_dom.contains(target_proc_ptr));
}

// Test 12: va_4k_valid requires specific bit patterns.
// va = 0 should NOT be a valid 4k VA (kernel address space check).
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

}
