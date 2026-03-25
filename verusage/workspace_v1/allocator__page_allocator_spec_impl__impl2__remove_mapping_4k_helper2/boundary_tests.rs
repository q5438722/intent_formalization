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
// Each test violates a precondition of remove_mapping_4k_helper2 or uses
// edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: Unaligned pointer (not 4k-aligned) is NOT page_ptr_valid.
// The precondition implies target_ptr must be page_ptr_valid (via
// mapped_pages_4k_wf). An odd pointer violates 4k alignment.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr_valid() {
    assert(page_ptr_valid(7usize));
}

// Test 2: Pointer at the first address beyond the valid range.
// NUM_PAGES * 4096 is 4k-aligned but exceeds the address space.
// SHOULD FAIL
proof fn test_boundary_ptr_overflow() {
    assert(page_ptr_valid((NUM_PAGES * 4096) as usize));
}

// Test 3: ref_count == 0 contradicts page being Mapped4k.
// The wf invariant mapped_pages_have_reference_counter says
// ref_count != 0 <==> Mapped state. With ref_count == 0 the
// page cannot be in mapped_pages_4k, contradicting precondition.
// SHOULD FAIL
proof fn test_boundary_ref_count_zero_with_mapping(
    mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    pcid: Pcid, va: VAddr,
    ref_count: usize
)
    requires
        ref_count == 0,
        mappings.finite(),
        io_mappings.finite(),
        mappings.contains((pcid, va)),
{
    // ref_count == mappings.len() + io_mappings.len() cannot hold
    // since mappings.len() >= 1 but ref_count == 0.
    assert(ref_count == mappings.len() + io_mappings.len());
}

// Test 4: ref_count == 2 violates the precondition ref_count == 1.
// With ref_count == 2, mappings.len() + io_mappings.len() == 2.
// Removing one mapping leaves ref_count at 1 (still mapped),
// so the page should NOT transition to Free4k. The function's
// behavior (push to free list, set Free4k) would be wrong.
// We encode this as: with 2 mappings and ref_count 2, removing
// one should NOT yield an empty set.
// SHOULD FAIL
proof fn test_boundary_ref_count_two_remove_yields_empty(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr,
)
    requires
        mappings.finite(),
        mappings.len() == 2,
        mappings.contains((pcid, va)),
{
    // After removing one of two elements, the set should have 1
    // element, not 0. Asserting it's empty should fail.
    assert(mappings.remove((pcid, va)).len() == 0);
}

// Test 5: Removing a mapping NOT present in the set is a no-op,
// but we assert the result changed (lost an element). This
// violates the precondition that (pcid, va) must be in mappings.
// SHOULD FAIL
proof fn test_boundary_remove_absent_mapping(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        mappings.finite(),
        !mappings.contains((pcid, va)),
        mappings.len() == 1,
{
    // Removing a non-member is identity; asserting result is empty
    // (as if we actually removed the element) should fail.
    assert(mappings.remove((pcid, va)).len() == 0);
}

}
