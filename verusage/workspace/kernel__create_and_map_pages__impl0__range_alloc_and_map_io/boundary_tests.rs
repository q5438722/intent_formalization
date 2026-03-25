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
pub type IOid = usize;

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

// Test 2: page_ptr2page_index with ptr = 0xFFF (just below page boundary).
// Not aligned, violates requires.
// SHOULD FAIL
proof fn test_boundary_page_ptr_just_below_boundary() {
    let ptr: usize = 0xFFF;
    assert(ptr % 0x1000 == 0);
}

// Test 3: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 4: page_index2page_ptr with i = usize::MAX overflows.
// SHOULD FAIL
proof fn test_boundary_page_index_overflow() {
    let i: usize = usize::MAX;
    assert(i < NUM_PAGES);
}

// Test 5: va_4k_valid with va = 0 should be false (kernel region constraint).
// The spec requires (va >> 39 & 0x1ff) >= KERNEL_MEM_END_L4INDEX (which is 1).
// For va = 0, (0 >> 39 & 0x1ff) == 0, which is < 1.
// Asserting it's valid should fail.
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    assert(spec_va_4k_valid(0usize));
}

// Test 6: va_4k_valid with va = 1 — not 4k-aligned.
// (1 & (!MEM_4k_MASK)) != 0, so not valid.
// SHOULD FAIL
proof fn test_boundary_va_not_aligned() {
    assert(spec_va_4k_valid(1usize));
}

// Test 7: create_entry_and_alloc_and_map_io requires mem_4k quota >= 4.
// If mem_4k == 3, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota_for_create_entry() {
    let mem_4k: usize = 3;
    assert(mem_4k >= 4usize);
}

// Test 8: create_entry_and_alloc_and_map_io requires free_pages >= 4.
// Having only 0 free pages violates this.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 4usize);
}

// Test 9: create_entry_and_alloc_and_map_io requires target_va is NOT
// already in the IO space domain.
// SHOULD FAIL
proof fn test_boundary_va_already_in_io_space(
    io_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        io_space_dom.contains(target_va),
{
    assert(io_space_dom.contains(target_va) == false);
}

// Test 10: range_alloc_and_map_io requires mem_4k >= 4 * va_range.len.
// If va_range.len = 10 and quota = 39, it's not enough (need 40).
// SHOULD FAIL
proof fn test_boundary_range_alloc_insufficient_quota() {
    let va_range_len: usize = 10;
    let mem_4k: usize = 39;
    assert(mem_4k >= 4 * va_range_len);
}

// Test 11: range_alloc_and_map_io requires va_range.len * 4 < usize::MAX.
// If va_range.len is very large, the overflow guard is violated.
// SHOULD FAIL
proof fn test_boundary_range_alloc_overflow_guard() {
    let va_range_len: usize = usize::MAX;
    assert(va_range_len * 4 < usize::MAX);
}

// Test 12: VaRange4K::index requires 0 <= i < self.len.
// Passing i == len violates the bound.
// SHOULD FAIL
proof fn test_boundary_varange_index_at_len() {
    let len: usize = 5;
    let i: usize = 5;
    assert(0 <= i && i < len);
}

}
