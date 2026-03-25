use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type VAddr = usize;
type PagePtr = usize;
type ContainerPtr = usize;
pub type PagePerm1g = PointsTo<[u8; PAGE_SZ_1g]>;
pub type PagePerm2m = PointsTo<[u8; PAGE_SZ_2m]>;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;
pub type IOid = usize;
pub type SLLIndex = i32;
pub type Pcid = usize;
pub const PAGE_SZ_4k: usize = 1usize << 12;
pub const PAGE_SZ_2m: usize = 1usize << 21;
pub const PAGE_SZ_1g: usize = 1usize << 30;
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
// Each test violates a precondition of alloc_page_4k or uses
// edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: An unaligned (odd) pointer cannot be page_ptr_valid.
// alloc_page_4k requires wf(), which constrains all free page pointers
// to satisfy page_ptr_valid (4k-aligned). An odd pointer violates alignment.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr_not_valid() {
    assert(page_ptr_valid(7usize));
}

// Test 2: A pointer beyond the valid address range is not page_ptr_valid.
// NUM_PAGES * 4096 is the first out-of-range page-aligned address.
// The spec should reject this as page_ptr_valid requires ptr/0x1000 < NUM_PAGES.
// SHOULD FAIL
proof fn test_boundary_out_of_range_ptr() {
    // NUM_PAGES == 2*1024*1024, so NUM_PAGES * 4096 == 0x200000000
    assert(page_ptr_valid((NUM_PAGES as usize * 4096) as usize));
}

// Test 3: Removing from an empty set should not produce a non-empty set.
// The precondition requires free_pages_4k.len() > 0.
// If we remove from an empty set, we should NOT get a positive-length result.
// SHOULD FAIL
proof fn test_boundary_remove_from_empty_set(p: PagePtr)
{
    let empty: Set<PagePtr> = Set::empty();
    assert(empty.remove(p).len() > 0);
}

// Test 4: A 2m-aligned pointer is NOT necessarily 4k-only-aligned.
// But a non-4k-aligned pointer (e.g., 0x1001) is never page_ptr_valid.
// Asserting a misaligned pointer at offset 1 is valid should fail.
// SHOULD FAIL
proof fn test_boundary_off_by_one_alignment() {
    // 0x1001 is 4097, not 4096-aligned
    assert(page_ptr_valid(0x1001usize));
}

// Test 5: A page_ptr_valid pointer does NOT have to be 2m-aligned.
// alloc_page_4k returns a 4k-valid pointer, not necessarily 2m-valid.
// Asserting all page_ptr_valid pointers are page_ptr_2m_valid is too strong.
// SHOULD FAIL
proof fn test_boundary_4k_valid_implies_2m_valid(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

}
