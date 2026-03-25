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
// Each test violates a precondition of alloc_page_4k_for_new_container
// or uses edge-case values to check if invalid inputs are properly rejected.
// All tests SHOULD FAIL verification.

// Test 1: An unaligned pointer (odd value) is not page_ptr_valid.
// alloc_page_4k_for_new_container ensures page_ptr_valid(ret.0), so only
// 4k-aligned pointers within range are valid. An odd value violates alignment.
// SHOULD FAIL
proof fn test_boundary_unaligned_ptr_not_valid() {
    assert(page_ptr_valid(7usize));
}

// Test 2: A pointer at exactly the upper boundary (NUM_PAGES * 4096) is invalid.
// page_ptr_valid requires ptr / 0x1000 < NUM_PAGES. At the boundary,
// ptr / 0x1000 == NUM_PAGES, which violates the strict inequality.
// SHOULD FAIL
proof fn test_boundary_out_of_range_ptr() {
    assert(page_ptr_valid((NUM_PAGES as usize * 4096) as usize));
}

// Test 3: Removing from an empty set cannot produce a non-empty set.
// The precondition requires free_pages_4k.len() > 0. If we try to remove
// from an empty set, the result should not have positive length.
// SHOULD FAIL
proof fn test_boundary_remove_from_empty_set(p: PagePtr)
{
    let empty: Set<PagePtr> = Set::empty();
    assert(empty.remove(p).len() > 0);
}

// Test 4: Off-by-one alignment (0x1001 = 4097) is not 4k-aligned.
// The spec requires page_ptr_valid which needs ptr % 0x1000 == 0.
// 4097 % 4096 == 1, so this must fail.
// SHOULD FAIL
proof fn test_boundary_off_by_one_alignment() {
    assert(page_ptr_valid(0x1001usize));
}

// Test 5: Inserting into a map and then looking up a DIFFERENT key should not
// find the inserted value's Set::empty(). The container_map postcondition
// inserts (ret.0, Set::empty()). Looking up an unrelated key should not yield empty set.
// SHOULD FAIL
proof fn test_boundary_map_insert_wrong_key(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    ret_ptr: ContainerPtr,
    other_key: ContainerPtr,
)
    requires
        !old_map.dom().contains(ret_ptr),
        !old_map.dom().contains(other_key),
        ret_ptr != other_key,
{
    let new_map = old_map.insert(ret_ptr, Set::empty());
    // other_key was never inserted, so it should not be in the domain
    assert(new_map.dom().contains(other_key));
}

// Test 6: A pointer at 0 is page_ptr_valid (0 % 0x1000 == 0 and 0 / 0x1000 == 0 < NUM_PAGES).
// But asserting that 0 is NOT valid should fail, confirming the spec accepts 0 as valid.
// This tests the lower boundary.
// SHOULD FAIL
proof fn test_boundary_zero_ptr_not_valid() {
    assert(!page_ptr_valid(0usize));
}

}
