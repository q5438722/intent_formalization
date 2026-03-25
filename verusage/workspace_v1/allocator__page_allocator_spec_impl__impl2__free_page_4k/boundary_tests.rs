use vstd::prelude::*;

fn main() {}

verus!{

pub type PagePtr = usize;
pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;

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
// Relevant: free_page_4k requires allocated_pages_4k().contains(target_ptr),
// which in turn requires page_ptr_valid(target_ptr) via allocated_pages_4k_wf.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(1usize));
}

// Test 2: page_index_valid rejects index == NUM_PAGES (off-by-one boundary).
// page_index_valid requires index < NUM_PAGES, so index == NUM_PAGES is invalid.
// Relevant: free_page_4k operates on page_ptr2page_index(target_ptr) which must be valid.
// SHOULD FAIL
proof fn test_boundary_page_index_at_limit() {
    assert(page_index_valid(NUM_PAGES));
}

// Test 3: page_ptr_valid does NOT hold for max usize (overflow boundary).
// usize::MAX is not 4k-aligned and its division overflows the page range.
// Relevant: target_ptr must be a valid page pointer in free_page_4k.
// SHOULD FAIL
proof fn test_boundary_max_usize_ptr() {
    assert(page_ptr_valid(usize::MAX));
}

// Test 4: A set after remove() should not still contain the removed element.
// Models: free_page_4k's precondition that target_ptr is in allocated_pages_4k,
// and postcondition that it is removed. Trying to call remove on a set
// that does NOT contain the element and still claim the element is present is wrong.
// SHOULD FAIL
proof fn test_boundary_remove_nonexistent(s: Set<PagePtr>, p: PagePtr)
    requires
        !s.contains(p),
{
    let new_s = s.remove(p);
    assert(new_s.contains(p));
}

// Test 5: page_ptr_2m_valid rejects a plain 4k-aligned pointer.
// 0x1000 (4096) is 4k-aligned but not 2M-aligned.
// Relevant: free_page_4k preserves free_pages_2m; only 2m-valid ptrs can be in that set.
// SHOULD FAIL
proof fn test_boundary_4k_ptr_not_2m_valid() {
    assert(page_ptr_2m_valid(0x1000usize));
}

}
