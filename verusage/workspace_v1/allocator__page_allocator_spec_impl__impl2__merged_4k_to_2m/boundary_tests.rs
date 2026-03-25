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

pub open spec fn page_index_1g_valid(i: usize) -> bool {
    &&& i % (512 * 512) as usize == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn spec_page_index_merge_2m_vaild(i: usize, j: usize) -> bool
    recommends
        page_index_2m_valid(i),
{
    i < j < i + 0x200
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}


// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// relevant to merged_4k_to_2m's requirements.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_2m_valid should reject 4k-aligned but not 2m-aligned pointers.
// 0x1000 (4096) is 4k-aligned but 4096 % 0x200000 != 0.
// Relevant: merged_4k_to_2m requires page_ptr_2m_valid(page_index2page_ptr(target_page_idx)).
// SHOULD FAIL
proof fn test_boundary_4k_not_2m_aligned() {
    assert(page_ptr_2m_valid(0x1000usize));
}

// Test 2: page_index_2m_valid should reject non-512-aligned indices.
// Index 1 satisfies 0 <= 1 < NUM_PAGES but 1 % 512 != 0.
// Relevant: merged_4k_to_2m needs target_page_idx to be 2m-aligned.
// SHOULD FAIL
proof fn test_boundary_index_not_512_aligned() {
    assert(page_index_2m_valid(1usize));
}

// Test 3: page_index_valid should reject index == NUM_PAGES (off-by-one).
// merged_4k_to_2m requires target_page_idx + 512 <= NUM_PAGES.
// SHOULD FAIL
proof fn test_boundary_index_at_num_pages() {
    assert(page_index_valid(NUM_PAGES));
}

// Test 4: spec_page_index_merge_2m_vaild should reject j at upper boundary.
// j must satisfy i < j < i + 0x200, so j == i + 0x200 (= 512) is out of range.
// Relevant: merged_4k_to_2m merges exactly 512 pages [i, i+512), j=512 is excluded.
// SHOULD FAIL
proof fn test_boundary_merge_index_at_upper_bound() {
    assert(spec_page_index_merge_2m_vaild(0usize, 512usize));
}

// Test 5: page_ptr_valid should reject unaligned pointer.
// ptr=1 has 1 % 0x1000 != 0.
// Relevant: all page pointers in the allocator must be 4k-aligned.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(1usize));
}

}
