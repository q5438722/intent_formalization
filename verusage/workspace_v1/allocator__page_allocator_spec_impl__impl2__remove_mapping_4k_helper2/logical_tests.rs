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

// Test 1: After removing the last mapping (ref_count==1, io_mappings empty),
// the new ref_count is 0, so the page CANNOT remain Mapped.
// Assert the opposite: ref_count is still > 0. Should fail.
// SHOULD FAIL
proof fn test_logical_page_still_mapped_after_last_remove(
    old_mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    pcid: Pcid, va: VAddr,
    ref_count: usize
)
    requires
        old_mappings.finite(),
        io_mappings.finite(),
        old_mappings.contains((pcid, va)),
        old_mappings.len() == 1,
        io_mappings.len() == 0,
        ref_count == old_mappings.len() + io_mappings.len(),
{
    let new_mappings = old_mappings.remove((pcid, va));
    let new_ref_count: int = (new_mappings.len() + io_mappings.len()) as int;
    // After removal, ref_count should be 0, so the page can't be Mapped.
    // Assert it's still > 0 — should fail.
    assert(new_ref_count > 0);
}

// Test 2: Two distinct valid pointers should NOT map to the same index.
// spec_page_ptr2page_index is injective on valid pointers.
// Assert two different valid pointers share the same index — should fail.
// SHOULD FAIL
proof fn test_logical_distinct_ptrs_same_index(p1: PagePtr, p2: PagePtr)
    requires
        page_ptr_valid(p1),
        page_ptr_valid(p2),
        p1 != p2,
{
    assert(spec_page_ptr2page_index(p1) == spec_page_ptr2page_index(p2));
}

// Test 3: Removing the sole element from a singleton set should yield
// an empty set. Assert the result is still non-empty — should fail.
// SHOULD FAIL
proof fn test_logical_singleton_remove_nonempty(
    s: Set<(Pcid, VAddr)>,
    x: (Pcid, VAddr)
)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(x),
{
    assert(s.remove(x).len() > 0);
}

// Test 4: page_ptr_valid (4k-aligned) does NOT imply page_ptr_2m_valid
// (2M-aligned). Asserting this stronger claim should fail.
// SHOULD FAIL
proof fn test_logical_4k_implies_2m(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 5: Removing a regular mapping should NOT cause io_mappings
// to become non-empty. Assert that io_mappings grew from 0 after
// a regular mapping removal — should fail.
// SHOULD FAIL
proof fn test_logical_remove_mapping_creates_io_mapping(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>
)
    requires
        old_io.finite(),
        old_io.len() == 0,
        new_io =~= old_io,
{
    assert(new_io.len() > 0);
}

}
