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
// to check if invalid inputs are properly rejected by alloc_and_map_io_4k.
// All tests SHOULD FAIL verification.

// Test 1: alloc_and_map_io_4k requires free_pages_4k.len() > 0.
// An empty free set cannot provide any element.
// SHOULD FAIL
proof fn test_boundary_empty_free_list_provides_element(
    free_set: Set<PagePtr>,
)
    requires
        free_set.len() == 0,
{
    assert(exists|p: PagePtr| free_set.contains(p) && page_ptr_valid(p));
}

// Test 2: A non-4096-aligned pointer violates page_ptr_valid.
// alloc_and_map_io_4k ensures page_ptr_valid(ret), so unaligned pointers must be rejected.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(0x1001usize));
}

// Test 3: page_ptr_valid rejects pointers at the exact NUM_PAGES boundary.
// ptr = NUM_PAGES * 4096 has ptr/0x1000 == NUM_PAGES (not < NUM_PAGES).
// SHOULD FAIL
proof fn test_boundary_ptr_at_max_boundary() {
    let ptr: usize = (NUM_PAGES * 4096) as usize;
    assert(page_ptr_valid(ptr));
}

// Test 4: alloc_and_map_io_4k requires container_map_4k@.dom().contains(c_ptr).
// If c_ptr is NOT in the domain, the call should be rejected.
// SHOULD FAIL
proof fn test_boundary_invalid_container_ptr(
    container_dom: Set<ContainerPtr>,
    c_ptr: ContainerPtr,
)
    requires
        !container_dom.contains(c_ptr),
{
    // Should not be able to derive that c_ptr is valid
    assert(container_dom.contains(c_ptr));
}

// Test 5: Zero pointer (0x0) is 4k-aligned but should still be a valid page ptr
// only if 0/0x1000 < NUM_PAGES (which is 0 < 2M, true). This tests the zero edge case.
// The spec DOES allow ptr=0 as valid. Asserting it is invalid SHOULD FAIL.
// SHOULD FAIL
proof fn test_boundary_zero_ptr_invalid() {
    assert(!page_ptr_valid(0usize));
}

}
