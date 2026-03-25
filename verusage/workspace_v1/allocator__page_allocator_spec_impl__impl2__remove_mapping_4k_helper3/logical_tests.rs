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

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
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


// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the specification,
// testing whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: With ref_count != 1, removing one mapping should NOT make ref_count 0.
// ref_count == mappings.len() + io_mappings.len(); with ref_count != 1 and
// mappings containing (pcid,va), after removal the new ref_count may still be > 0.
// But it is NOT guaranteed to be 0. Assert it IS 0 — should fail.
// SHOULD FAIL
proof fn test_logical_ref_count_becomes_zero(
    old_mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    pcid: Pcid, va: VAddr,
    ref_count: usize
)
    requires
        old_mappings.finite(),
        io_mappings.finite(),
        old_mappings.contains((pcid, va)),
        ref_count == old_mappings.len() + io_mappings.len(),
        ref_count != 1,
{
    let new_mappings = old_mappings.remove((pcid, va));
    let new_ref_count: int = (new_mappings.len() + io_mappings.len()) as int;
    // Since ref_count != 1, new_ref_count could be >= 1 or == 0 (if ref_count==2
    // and io_mappings==0 and mappings had 2 elements). But asserting it's always 0
    // is wrong in general.
    assert(new_ref_count == 0);
}

// Test 2: Two distinct valid pointers should NOT have the same page index.
// page_ptr2page_index is injective on valid pointers.
// Assert two different valid pointers map to the same index — should fail.
// SHOULD FAIL
proof fn test_logical_distinct_ptrs_same_index(p1: PagePtr, p2: PagePtr)
    requires
        page_ptr_valid(p1),
        page_ptr_valid(p2),
        p1 != p2,
{
    assert(spec_page_ptr2page_index(p1) == spec_page_ptr2page_index(p2));
}

// Test 3: Removing from a set of size > 1 yields an empty set.
// With ref_count != 1, the mappings+io_mappings sum is > 1, so after removing
// one mapping the set should still be non-empty. Assert it becomes empty.
// SHOULD FAIL
proof fn test_logical_multi_element_remove_yields_empty(
    s: Set<(Pcid, VAddr)>,
    x: (Pcid, VAddr),
    y: (Pcid, VAddr)
)
    requires
        s.finite(),
        s.len() == 2,
        s.contains(x),
        s.contains(y),
        x !== y,
{
    assert(s.remove(x).len() == 0);
}

// Test 4: page_ptr_valid does NOT imply page_ptr_2m_valid.
// The spec constrains target_ptr to be page_ptr_valid (4k-aligned).
// Asserting it must also be 2M-aligned is a stronger, unwarranted claim.
// SHOULD FAIL
proof fn test_logical_4k_implies_2m(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 5: The mapping set size cannot increase after a remove operation.
// Assert the new set is LARGER than the old set — should fail.
// SHOULD FAIL
proof fn test_logical_remove_increases_size(
    old_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        old_mappings.finite(),
        old_mappings.contains((pcid, va)),
        old_mappings.len() >= 2,
{
    let new_mappings = old_mappings.remove((pcid, va));
    assert(new_mappings.len() > old_mappings.len());
}

}
