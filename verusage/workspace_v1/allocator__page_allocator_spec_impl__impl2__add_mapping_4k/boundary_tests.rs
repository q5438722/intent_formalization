use vstd::prelude::*;

fn main() {}

verus!{

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_index_2m_valid(i: usize) -> bool {
    &&& i % 512 == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}


// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_valid rejects non-4096-aligned pointers.
// ptr=1 has ptr % 0x1000 = 1 != 0, so page_ptr_valid(1) is false.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(1usize));
}

// Test 2: page_index_valid rejects index == NUM_PAGES (off-by-one boundary).
// page_index_valid requires index < NUM_PAGES, so index == NUM_PAGES is invalid.
// SHOULD FAIL
proof fn test_boundary_page_index_at_limit() {
    assert(page_index_valid(NUM_PAGES));
}

// Test 3: page_index_2m_valid rejects non-512-aligned valid page index.
// Index 1 is valid (< NUM_PAGES) but 1 % 512 != 0.
// Relevant to add_mapping_4k which operates in the 4k page context.
// SHOULD FAIL
proof fn test_boundary_non_512_aligned_index() {
    assert(page_index_2m_valid(1usize));
}

// Test 4: page_ptr_2m_valid rejects non-2M-aligned valid 4k pointer.
// 0x1000 (4096) is 4k-aligned but 4096 % 0x200000 != 0.
// SHOULD FAIL
proof fn test_boundary_4k_valid_not_2m_valid() {
    assert(page_ptr_2m_valid(0x1000usize));
}

// Test 5: page_ptr_1g_valid rejects non-1G-aligned valid 2M pointer.
// 0x200000 (2M) is 2M-aligned but 0x200000 % 0x40000000 != 0.
// SHOULD FAIL
proof fn test_boundary_2m_valid_not_1g_valid() {
    assert(page_ptr_1g_valid(0x200000usize));
}

}
