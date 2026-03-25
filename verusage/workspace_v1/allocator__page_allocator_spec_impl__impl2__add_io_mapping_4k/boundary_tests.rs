use vstd::prelude::*;

fn main() {}

verus!{

pub type VAddr = usize;
pub type PagePtr = usize;
pub type IOid = usize;
pub type Pcid = usize;

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
// relevant to add_io_mapping_4k to check if invalid inputs
// are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_valid rejects non-aligned pointers.
// add_io_mapping_4k requires target_ptr to be in mapped_pages_4k,
// which requires page_ptr_valid. ptr=1 is not 4096-aligned.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(1usize));
}

// Test 2: page_index_valid rejects index exactly at NUM_PAGES.
// add_io_mapping_4k internally uses page_ptr2page_index which
// must produce a valid index. NUM_PAGES is the exclusive boundary.
// SHOULD FAIL
proof fn test_boundary_page_index_at_limit() {
    assert(page_index_valid(NUM_PAGES));
}

// Test 3: Overflow boundary — a pointer whose page index would
// equal NUM_PAGES. ptr = NUM_PAGES * 4096 means ptr/4096 == NUM_PAGES,
// so page_ptr_valid should be false.
// SHOULD FAIL
proof fn test_boundary_page_ptr_at_max() {
    assert(page_ptr_valid((NUM_PAGES * 4096) as usize));
}

// Test 4: An io_mapping that already exists violates the precondition
// !old.page_io_mappings(target).contains((ioid,va)).
// Asserting a duplicate is absent from a set that contains it should fail.
// SHOULD FAIL
proof fn test_boundary_duplicate_io_mapping(
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        io_mappings.contains((ioid, va)),
{
    assert(!io_mappings.contains((ioid, va)));
}

// Test 5: The combined ref_count bound requires
// mappings.len() + io_mappings.len() < usize::MAX.
// If both are at maximum, the sum overflows — this boundary must be rejected.
// SHOULD FAIL
proof fn test_boundary_ref_count_at_max(
    mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
)
    requires
        mappings.finite(),
        io_mappings.finite(),
        mappings.len() + io_mappings.len() >= usize::MAX,
{
    assert(mappings.len() + io_mappings.len() < usize::MAX);
}

}
