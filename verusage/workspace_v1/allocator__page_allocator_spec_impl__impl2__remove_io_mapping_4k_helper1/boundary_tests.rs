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
// Each test violates a precondition of remove_io_mapping_4k_helper1 or uses
// edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: IO mapping does NOT exist in the set, yet we assert removal yields empty.
// Precondition requires page_io_mappings(target_ptr).contains((ioid, va)).
// Violating this: the mapping is absent.
// SHOULD FAIL
proof fn test_boundary_remove_absent_io_mapping(
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        io_mappings.finite(),
        !io_mappings.contains((ioid, va)),
        io_mappings.len() == 1,
{
    // If (ioid, va) is not in the set, removing it is a no-op,
    // but we assert the set becomes empty (as if it were the element).
    assert(io_mappings.remove((ioid, va)).len() == 0);
}

// Test 2: ref_count == 0 contradicts having a non-empty io_mappings set.
// Precondition requires ref_count == 1. The wf invariant says
// ref_count == mappings.len() + io_mappings.len().
// With ref_count == 0 and non-empty io_mappings, this is inconsistent.
// SHOULD FAIL
proof fn test_boundary_ref_count_zero_with_io_mapping(
    mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr,
    ref_count: usize
)
    requires
        ref_count == 0,
        mappings.finite(),
        io_mappings.finite(),
        io_mappings.contains((ioid, va)),
{
    // ref_count should equal mappings.len() + io_mappings.len(), but
    // io_mappings is non-empty (len >= 1) while ref_count is 0.
    assert(ref_count == mappings.len() + io_mappings.len());
}

// Test 3: Unaligned pointer (not 4k-aligned) is NOT page_ptr_valid.
// The precondition implies target_ptr must be page_ptr_valid
// (via mapped_pages_4k_wf). Asserting an odd pointer is valid should fail.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr_valid() {
    assert(page_ptr_valid(7usize));
}

// Test 4: Pointer exceeding address space is NOT page_ptr_valid.
// NUM_PAGES * 4096 is out of range.
// SHOULD FAIL
proof fn test_boundary_ptr_exceeding_address_space() {
    let ptr: usize = (NUM_PAGES * 4096) as usize;
    assert(page_ptr_valid(ptr));
}

// Test 5: ref_count == 2 with a single io_mapping contradicts ref_count == 1.
// The wf invariant requires ref_count == mappings.len() + io_mappings.len().
// With exactly 1 io_mapping and 0 mappings, ref_count must be 1, not 2.
// SHOULD FAIL
proof fn test_boundary_ref_count_2_single_io_mapping(
    mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr,
    ref_count: usize
)
    requires
        ref_count == 2,
        mappings.finite(),
        io_mappings.finite(),
        mappings.len() == 0,
        io_mappings.len() == 1,
        io_mappings.contains((ioid, va)),
{
    assert(ref_count == mappings.len() + io_mappings.len());
}

// Test 6: A pointer at exactly 0 is page_ptr_valid (0 % 0x1000 == 0
// and 0 / 0x1000 == 0 < NUM_PAGES). However, the is_io_page flag
// being false while claiming true should be rejected.
// Here we test that is_io_page == false contradicts the precondition is_io_page == true.
// SHOULD FAIL
proof fn test_boundary_is_io_page_false_contradicts_precondition(
    is_io_page: bool
)
    requires
        is_io_page == false,
{
    // Precondition requires is_io_page == true
    assert(is_io_page == true);
}

}
