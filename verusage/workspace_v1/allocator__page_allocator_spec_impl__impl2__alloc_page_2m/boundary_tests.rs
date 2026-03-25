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
pub const MAX_USIZE: u64 = 31 * 1024 * 1024 * 1024;
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
// Each test violates a precondition of alloc_page_2m or uses
// edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: Asserting that a page pointer with zero value is page_ptr_2m_valid.
// ptr == 0 is technically 2m-aligned (0 % 0x200000 == 0) and 0/4096 == 0 < NUM_PAGES.
// However, asserting it is NOT 2m-valid should fail since 0 actually satisfies the spec.
// Instead, we test that a non-2m-aligned but 4k-aligned pointer is 2m-valid — SHOULD FAIL.
// SHOULD FAIL
proof fn test_boundary_4k_aligned_not_2m_valid() {
    // 0x1000 == 4096, which is 4k-aligned but NOT 2m-aligned (0x200000 == 2097152)
    assert(page_ptr_2m_valid(0x1000usize));
}

// Test 2: Asserting an unaligned (odd) pointer is page_ptr_valid.
// Precondition for alloc_page_2m requires wf() which constrains all pointers
// in free_pages_2m to be page_ptr_2m_valid, hence page_ptr_valid.
// An odd pointer violates alignment.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr() {
    assert(page_ptr_valid(7usize));
}

// Test 3: Asserting a pointer beyond valid address range is page_ptr_2m_valid.
// NUM_PAGES * 4096 is the first invalid page-aligned address.
// This tests that the spec correctly rejects out-of-range pointers.
// SHOULD FAIL
proof fn test_boundary_overflow_2m_ptr() {
    // NUM_PAGES == 2*1024*1024 == 0x200000. 0x200000 * 4096 = 0x200000000
    // This pointer is 2m-aligned but out of range
    assert(page_ptr_2m_valid((NUM_PAGES as usize * 4096) as usize));
}

// Test 4: An empty free set (len == 0) should not allow allocation.
// The precondition requires free_pages_2m.len() > 0.
// After removing from a set, claiming the result has more elements than the original
// should fail.
// SHOULD FAIL
proof fn test_boundary_remove_from_empty_set(p: PagePtr)
{
    let empty: Set<PagePtr> = Set::empty();
    // Removing from empty should not produce non-empty
    assert(empty.remove(p).len() > 0);
}

// Test 5: A valid 4k pointer does NOT have to be 2m-aligned.
// The spec requires alloc_page_2m to return a 2m-valid pointer (from free_pages_2m).
// Asserting that any page_ptr_valid pointer is also page_ptr_2m_valid is too strong.
// SHOULD FAIL
proof fn test_boundary_4k_valid_implies_2m_valid(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

}
