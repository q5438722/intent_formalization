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
// Each test violates a precondition of remove_mapping_4k_helper3 or uses
// edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: Removing a mapping that does NOT exist in the set.
// Precondition requires page_mappings(target_ptr).contains((pcid, va)).
// Here the mapping is absent, yet we assert removal shrinks the set.
// SHOULD FAIL
proof fn test_boundary_remove_absent_mapping(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        mappings.finite(),
        !mappings.contains((pcid, va)),
        mappings.len() == 2,
{
    // Removing a non-existent element should NOT decrease length.
    assert(mappings.remove((pcid, va)).len() == 1);
}

// Test 2: ref_count == 1 violates the helper3 precondition (ref_count != 1).
// With ref_count == 1 and mappings.len()==1, io_mappings.len()==0:
// after removal ref_count would become 0, meaning the page should leave
// Mapped4k state. But helper3 assumes this won't happen.
// We assert the impossible: ref_count 1 is compatible with ref_count != 1.
// SHOULD FAIL
proof fn test_boundary_ref_count_is_one()
{
    let ref_count: usize = 1;
    assert(ref_count != 1);
}

// Test 3: Unaligned pointer (not 4k-aligned) is NOT page_ptr_valid.
// The precondition implies target_ptr must be page_ptr_valid via mapped_pages_4k.
// Asserting that an odd pointer is valid should fail.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr_valid() {
    assert(page_ptr_valid(7usize));
}

// Test 4: Pointer exceeding address space is NOT page_ptr_valid.
// NUM_PAGES * 4096 is the first invalid page-aligned address.
// SHOULD FAIL
proof fn test_boundary_ptr_overflow() {
    assert(page_ptr_valid((NUM_PAGES * 4096) as usize));
}

// Test 5: Removing from an empty set cannot produce a non-empty result.
// Precondition requires page_mappings contains (pcid, va), so mappings is non-empty.
// Here we violate that: start with empty mappings and assert removing yields something.
// SHOULD FAIL
proof fn test_boundary_remove_from_empty(
    pcid: Pcid, va: VAddr
) {
    let empty: Set<(Pcid, VAddr)> = Set::empty();
    assert(empty.remove((pcid, va)).len() > 0);
}

}
