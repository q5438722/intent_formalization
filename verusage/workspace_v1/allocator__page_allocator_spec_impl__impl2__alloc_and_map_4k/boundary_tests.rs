use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type PagePtr = usize;

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
// to check if invalid inputs are properly rejected by alloc_and_map_4k.
// All tests SHOULD FAIL verification.

// Test 1: alloc_and_map_4k requires free_pages_4k.len() > 0.
// A sequence of length 0 cannot provide an element to pop.
// Asserting we can still obtain a valid element from an empty set should fail.
// SHOULD FAIL
proof fn test_boundary_empty_free_list_provides_element(
    free_set: Set<PagePtr>,
)
    requires
        free_set.len() == 0,
{
    // An empty free set should not contain any valid page ptr
    assert(exists|p: PagePtr| free_set.contains(p) && page_ptr_valid(p));
}

// Test 2: alloc_and_map_4k ensures page_ptr_valid(ret).
// A non-4096-aligned pointer violates page_ptr_valid.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr_as_ret() {
    // ptr=0x1001 is not 4k-aligned, so it cannot be a valid return
    assert(page_ptr_valid(0x1001usize));
}

// Test 3: page_ptr_valid rejects pointers beyond NUM_PAGES boundary.
// ptr = NUM_PAGES * 4096 is exactly at the boundary (ptr/0x1000 == NUM_PAGES, not <).
// SHOULD FAIL
proof fn test_boundary_ptr_at_max_limit() {
    assert(page_ptr_valid((NUM_PAGES * 4096) as usize));
}

// Test 4: alloc_and_map_4k requires container_map_4k.dom().contains(c_ptr).
// If the container map domain is empty, no c_ptr can satisfy the precondition.
// Asserting that an empty domain contains some element should fail.
// SHOULD FAIL
proof fn test_boundary_empty_container_domain(
    dom: Set<usize>,
    c_ptr: usize,
)
    requires
        dom.finite(),
        dom.len() == 0,
{
    assert(dom.contains(c_ptr));
}

// Test 5: page_ptr_valid(0) holds (0 % 0x1000 == 0, 0/0x1000 == 0 < NUM_PAGES).
// But asserting page_ptr_1g_valid(0) is also a separate check.
// ptr=1 is neither 4k-valid nor any other validity. Attempting to treat it as valid should fail.
// SHOULD FAIL
proof fn test_boundary_ptr_one_invalid() {
    assert(page_ptr_valid(1usize));
}

}
