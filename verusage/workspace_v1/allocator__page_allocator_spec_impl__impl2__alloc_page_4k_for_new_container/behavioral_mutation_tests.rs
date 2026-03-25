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

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs matching the postconditions of
// alloc_page_4k_for_new_container and mutates expected outputs or relations.
// All tests SHOULD FAIL verification.

// Test 1: After allocation, the returned page SHOULD be removed from free_pages_4k.
// Postcondition: free_pages_4k() =~= old.free_pages_4k().remove(ret.0).
// Mutated: assert the page is STILL in the free set.
// SHOULD FAIL
proof fn test_mutation_page_still_free(
    old_free: Set<PagePtr>,
    new_free: Set<PagePtr>,
    ret_ptr: PagePtr,
)
    requires
        old_free.contains(ret_ptr),
        old_free.finite(),
        new_free =~= old_free.remove(ret_ptr),
{
    assert(new_free.contains(ret_ptr));
}

// Test 2: After allocation, ret.0 SHOULD be in allocated_pages_4k.
// Postcondition: allocated_pages_4k() =~= old.allocated_pages_4k().insert(ret.0).
// Mutated: assert ret.0 is NOT in the allocated set.
// SHOULD FAIL
proof fn test_mutation_page_not_allocated(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret_ptr: PagePtr,
)
    requires
        !old_alloc.contains(ret_ptr),
        old_alloc.finite(),
        new_alloc =~= old_alloc.insert(ret_ptr),
{
    assert(!new_alloc.contains(ret_ptr));
}

// Test 3: The new container's owned pages SHOULD be empty.
// Postcondition: get_container_owned_pages(ret.0) == Set::<PagePtr>::empty().
// Mutated: assert the new container's owned pages are non-empty.
// SHOULD FAIL
proof fn test_mutation_new_container_pages_nonempty(
    container_map: Map<ContainerPtr, Set<PagePtr>>,
    ret_ptr: ContainerPtr,
)
    requires
        !container_map.dom().contains(ret_ptr),
{
    let new_map = container_map.insert(ret_ptr, Set::empty());
    // The new container should have empty pages, but we assert non-empty
    assert(new_map[ret_ptr].len() > 0);
}

// Test 4: Existing containers' owned pages SHOULD be preserved.
// Postcondition: for all c in old container_map dom, owned_pages(c) unchanged.
// Mutated: assert an existing container lost all its pages.
// SHOULD FAIL
proof fn test_mutation_existing_container_pages_changed(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    ret_ptr: ContainerPtr,
    existing_c: ContainerPtr,
    existing_pages: Set<PagePtr>,
)
    requires
        old_map.dom().contains(existing_c),
        existing_c != ret_ptr,
        old_map[existing_c] == existing_pages,
        existing_pages.len() > 0,
        existing_pages.finite(),
{
    let new_map = old_map.insert(ret_ptr, Set::empty());
    // Existing container's pages should be preserved, but we assert they are now empty
    assert(new_map[existing_c] == Set::<PagePtr>::empty());
}

// Test 5: The free list length SHOULD decrease by exactly 1.
// Postcondition: self.free_pages_4k.len() == old(self).free_pages_4k.len() - 1.
// Mutated: assert the length stayed the same.
// SHOULD FAIL
proof fn test_mutation_free_list_length_unchanged(
    old_len: nat,
    new_len: nat,
)
    requires
        old_len > 0,
        new_len == (old_len - 1) as nat,
{
    assert(new_len == old_len);
}

// Test 6: container_map_4k should have ret.0 in its domain after allocation.
// Postcondition: container_map_4k =~= old.container_map_4k.insert(ret.0, Set::empty()).
// Mutated: assert ret.0 is NOT in the new domain.
// SHOULD FAIL
proof fn test_mutation_container_map_missing_new_entry(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    ret_ptr: ContainerPtr,
)
    requires
        !old_map.dom().contains(ret_ptr),
{
    let new_map = old_map.insert(ret_ptr, Set::empty());
    assert(!new_map.dom().contains(ret_ptr));
}

// Test 7: mapped_pages_4k should be UNCHANGED after allocation.
// Postcondition: mapped_pages_4k() =~= old.mapped_pages_4k().
// Mutated: assert the returned page is now mapped.
// SHOULD FAIL
proof fn test_mutation_ret_page_now_mapped(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret_ptr: PagePtr,
)
    requires
        new_mapped =~= old_mapped,
        !old_mapped.contains(ret_ptr),
{
    assert(new_mapped.contains(ret_ptr));
}

}
