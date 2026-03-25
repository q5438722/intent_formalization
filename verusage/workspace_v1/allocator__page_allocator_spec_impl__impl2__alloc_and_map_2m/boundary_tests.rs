use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type PagePtr = usize;
pub type ContainerPtr = usize;

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
// to check if invalid inputs are properly rejected by alloc_and_map_2m.
// All tests SHOULD FAIL verification.

// Test 1: alloc_and_map_2m requires free_pages_2m.len() > 0.
// An empty free set cannot provide any page.
// SHOULD FAIL
proof fn test_boundary_empty_free_2m_list_provides_element(
    free_set: Set<PagePtr>,
)
    requires
        free_set.len() == 0,
{
    // An empty set should not contain any valid 2m page ptr
    assert(exists|p: PagePtr| free_set.contains(p) && page_ptr_2m_valid(p));
}

// Test 2: alloc_and_map_2m returns a page_ptr_2m_valid pointer.
// A non-2m-aligned pointer violates page_ptr_2m_valid.
// ptr=0x1000 is 4k-aligned but not 2m-aligned.
// SHOULD FAIL
proof fn test_boundary_4k_aligned_not_2m_valid() {
    assert(page_ptr_2m_valid(0x1000usize));
}

// Test 3: page_ptr_2m_valid rejects pointers beyond NUM_PAGES boundary.
// ptr = NUM_PAGES * 4096 is at the boundary (ptr/4096 == NUM_PAGES, not <).
// SHOULD FAIL
proof fn test_boundary_2m_ptr_at_max_limit() {
    assert(page_ptr_2m_valid((NUM_PAGES * 4096) as usize));
}

// Test 4: alloc_and_map_2m requires container_map_2m.dom().contains(c_ptr).
// If the container map domain is empty, no c_ptr satisfies the precondition.
// SHOULD FAIL
proof fn test_boundary_empty_container_2m_domain(
    dom: Set<ContainerPtr>,
    c_ptr: ContainerPtr,
)
    requires
        dom.finite(),
        dom.len() == 0,
{
    assert(dom.contains(c_ptr));
}

// Test 5: A non-aligned pointer (odd value) cannot be page_ptr_2m_valid.
// ptr=1 fails both alignment checks.
// SHOULD FAIL
proof fn test_boundary_ptr_one_not_2m_valid() {
    assert(page_ptr_2m_valid(1usize));
}

}
