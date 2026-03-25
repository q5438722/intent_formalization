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

// Test 1: After removing the last IO mapping (ref_count drops from 1 to 0),
// the page should NOT remain mapped. The wf invariant says
// ref_count != 0 <==> Mapped state. With ref_count == 0 the page
// is no longer Mapped4k. Assert ref_count is still positive — should fail.
// SHOULD FAIL
proof fn test_logical_page_still_mapped_after_last_io_remove(
    mappings: Set<(Pcid, VAddr)>,
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr,
    ref_count: usize
)
    requires
        mappings.finite(),
        io_mappings.finite(),
        mappings.len() == 0,
        io_mappings.contains((ioid, va)),
        io_mappings.len() == 1,
        ref_count == mappings.len() + io_mappings.len(),
{
    let new_io = io_mappings.remove((ioid, va));
    let new_ref_count: int = (mappings.len() + new_io.len()) as int;
    // After removal, ref_count should be 0, so page can't be Mapped.
    // Assert it's still > 0 — should fail.
    assert(new_ref_count > 0);
}

// Test 2: The spec does NOT guarantee target_ptr becomes a free page
// after removal. It transitions to Unavailable4k, not Free4k.
// Assert that after IO mapping removal the page is in free_pages_4k — should fail.
// We model this by checking the page_ptr would need to be in a "free" set.
// SHOULD FAIL
proof fn test_logical_target_becomes_free_page(
    free_set: Set<PagePtr>,
    mapped_set: Set<PagePtr>,
    target_ptr: PagePtr
)
    requires
        mapped_set.contains(target_ptr),
        !free_set.contains(target_ptr),
        free_set.finite(),
        mapped_set.finite(),
{
    // The page was mapped and not free. Removing the mapping does
    // NOT add it to the free set. Assert it does — should fail.
    let new_mapped = mapped_set.remove(target_ptr);
    assert(free_set.contains(target_ptr));
}

// Test 3: Two distinct valid pointers should NOT have the same page index.
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

// Test 4: Removing the only element from a singleton set yields a non-empty set.
// The spec relies on Set::remove producing empty when applied to a singleton.
// This tests the foundational assumption — should fail.
// SHOULD FAIL
proof fn test_logical_singleton_io_remove_nonempty(
    s: Set<(IOid, VAddr)>,
    x: (IOid, VAddr)
)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(x),
{
    assert(s.remove(x).len() > 0);
}

// Test 5: The spec does NOT guarantee that target_ptr ends up in
// allocated_pages_4k after removal. The page goes to Unavailable4k,
// which is distinct from Allocated4k.
// Assert the target appears in allocated_pages_4k — should fail.
// SHOULD FAIL
proof fn test_logical_target_becomes_allocated(
    alloc_set: Set<PagePtr>,
    target_ptr: PagePtr
)
    requires
        !alloc_set.contains(target_ptr),
        alloc_set.finite(),
{
    // The spec preserves allocated_pages_4k unchanged.
    // If target wasn't allocated before, it isn't after.
    assert(alloc_set.contains(target_ptr));
}

// Test 6: The spec does NOT guarantee that mapped_pages_4k becomes empty
// after one removal. There may be other mapped pages.
// Assert mapped_pages_4k is empty after removal — should fail.
// SHOULD FAIL
proof fn test_logical_mapped_set_empty_after_removal(
    mapped_set: Set<PagePtr>,
    target_ptr: PagePtr,
    other_ptr: PagePtr
)
    requires
        mapped_set.contains(target_ptr),
        mapped_set.contains(other_ptr),
        target_ptr != other_ptr,
        mapped_set.finite(),
{
    let new_mapped = mapped_set.remove(target_ptr);
    // Other pages are still mapped, so set can't be empty.
    assert(new_mapped.len() == 0);
}

}
