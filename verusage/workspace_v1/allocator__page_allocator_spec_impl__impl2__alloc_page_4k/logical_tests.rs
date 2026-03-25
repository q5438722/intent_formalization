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

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
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

// Test 1: Determinism — two valid choices from the same free set must be equal.
// The spec does NOT guarantee alloc_page_4k is deterministic; any page from the
// free set could be returned. Asserting two different valid candidates must be
// the same should fail.
// SHOULD FAIL
proof fn test_logical_determinism(
    free_set: Set<PagePtr>,
    ret1: PagePtr,
    ret2: PagePtr
)
    requires
        free_set.finite(),
        free_set.len() > 1,
        free_set.contains(ret1),
        free_set.contains(ret2),
        ret1 != ret2,
{
    // Assert the two different choices must be the same
    assert(ret1 == ret2);
}

// Test 2: page_ptr_valid does NOT imply page_ptr_2m_valid.
// alloc_page_4k ensures page_ptr_valid(ret.0), but this does not imply 2m alignment.
// SHOULD FAIL
proof fn test_logical_4k_implies_2m(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 3: Two distinct page_ptr_valid pointers map to the same page index.
// spec_page_ptr2page_index should be injective on valid pointers.
// Asserting two different valid pointers yield the same index should fail.
// SHOULD FAIL
proof fn test_logical_distinct_ptrs_same_index(p1: PagePtr, p2: PagePtr)
    requires
        page_ptr_valid(p1),
        page_ptr_valid(p2),
        p1 != p2,
{
    assert(spec_page_ptr2page_index(p1) == spec_page_ptr2page_index(p2));
}

// Test 4: Removing one element from a set of size > 1 does NOT empty it.
// The spec says free_pages_4k() =~= old.free_pages_4k().remove(ret.0).
// Asserting the set becomes empty when it had 2 elements should fail.
// SHOULD FAIL
proof fn test_logical_remove_empties_larger_set(
    free_set: Set<PagePtr>,
    ret_ptr: PagePtr
)
    requires
        free_set.finite(),
        free_set.len() == 2,
        free_set.contains(ret_ptr),
{
    let new_set = free_set.remove(ret_ptr);
    // Assert removing one of two produces empty set
    assert(new_set.len() == 0);
}

// Test 5: Allocation should not cause spurious pages to appear in allocated set.
// If a page was NOT in old allocated_pages_4k, and it's NOT the returned page,
// then it should NOT appear in the new allocated set.
// SHOULD FAIL
proof fn test_logical_spurious_allocation(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret_ptr: PagePtr,
    other_ptr: PagePtr
)
    requires
        old_alloc.finite(),
        new_alloc =~= old_alloc.insert(ret_ptr),
        !old_alloc.contains(other_ptr),
        other_ptr != ret_ptr,
{
    // Assert a different page also appeared in the allocated set
    assert(new_alloc.contains(other_ptr));
}

// Test 6: Container owned pages are preserved, so no container gains the new page.
// The spec ensures forall containers c: get_container_owned_pages(c) unchanged.
// Asserting a container gained ownership of a new page should fail.
// SHOULD FAIL
proof fn test_logical_container_gains_page(
    old_owned: Set<PagePtr>,
    new_owned: Set<PagePtr>,
    phantom_page: PagePtr
)
    requires
        new_owned =~= old_owned,
        !old_owned.contains(phantom_page),
{
    // Assert a container gained a page it didn't own before
    assert(new_owned.contains(phantom_page));
}

}
