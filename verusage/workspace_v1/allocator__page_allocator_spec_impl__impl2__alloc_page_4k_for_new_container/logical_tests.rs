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

// Test 1: Determinism — two valid candidates from the same free set must be equal.
// The spec does NOT guarantee determinism; any page from the free set could be returned.
// Asserting two different valid candidates must be the same should fail.
// SHOULD FAIL
proof fn test_logical_determinism(
    free_set: Set<PagePtr>,
    ret1: PagePtr,
    ret2: PagePtr,
)
    requires
        free_set.finite(),
        free_set.len() > 1,
        free_set.contains(ret1),
        free_set.contains(ret2),
        ret1 != ret2,
{
    assert(ret1 == ret2);
}

// Test 2: page_ptr_valid does NOT imply page_ptr_2m_valid.
// alloc_page_4k_for_new_container ensures page_ptr_valid(ret.0), but the returned
// page is 4k-aligned, not necessarily 2m-aligned.
// SHOULD FAIL
proof fn test_logical_4k_implies_2m(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 3: Two distinct page_ptr_valid pointers CANNOT map to the same index.
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

// Test 4: Removing one element from a set of size 2 does NOT empty it.
// The spec says free_pages_4k() =~= old.free_pages_4k().remove(ret.0).
// After removing one element from a size-2 set, size should be 1, not 0.
// SHOULD FAIL
proof fn test_logical_remove_empties_larger_set(
    free_set: Set<PagePtr>,
    ret_ptr: PagePtr,
)
    requires
        free_set.finite(),
        free_set.len() == 2,
        free_set.contains(ret_ptr),
{
    let new_set = free_set.remove(ret_ptr);
    assert(new_set.len() == 0);
}

// Test 5: The allocated page (ret.0) should NOT also be in free_pages_2m.
// The spec says free_pages_2m is unchanged. But we try to assert that
// a page_ptr_valid pointer must be in some 2m set, which is not guaranteed.
// SHOULD FAIL
proof fn test_logical_alloc_4k_page_in_2m_free(
    free_2m: Set<PagePtr>,
    ret_ptr: PagePtr,
)
    requires
        page_ptr_valid(ret_ptr),
        free_2m.finite(),
{
    assert(free_2m.contains(ret_ptr));
}

// Test 6: Container map insertion should NOT affect other container sizes.
// We try to assert a stronger property: inserting a new container causes
// ALL existing containers to have exactly 0 pages. This is not entailed.
// SHOULD FAIL
proof fn test_logical_insert_clears_all_containers(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    ret_ptr: ContainerPtr,
    other: ContainerPtr,
)
    requires
        old_map.dom().contains(other),
        other != ret_ptr,
        old_map[other].len() > 0,
        old_map[other].finite(),
{
    let new_map = old_map.insert(ret_ptr, Set::empty());
    // Stronger claim: all containers now have empty pages
    assert(new_map[other].len() == 0);
}

// Test 7: The spec does not guarantee that the returned pointer is non-zero.
// page_ptr_valid(0) is actually true (0 % 0x1000 == 0 && 0/0x1000 < NUM_PAGES).
// Asserting that all page_ptr_valid pointers are non-zero is too strong.
// SHOULD FAIL
proof fn test_logical_valid_ptr_nonzero(ptr: PagePtr)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr != 0);
}

}
