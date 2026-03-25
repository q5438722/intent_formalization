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
// Each test starts from valid inputs (matching alloc_page_4k postconditions)
// and mutates expected outputs or relations.
// All tests SHOULD FAIL verification.

// Test 1: After allocation, the returned page should be REMOVED from free_pages_4k.
// Postcondition: free_pages_4k() =~= old.free_pages_4k().remove(ret.0).
// Mutated: assert the page is STILL in the free set after allocation.
// SHOULD FAIL
proof fn test_mutation_page_still_free(
    old_free: Set<PagePtr>,
    new_free: Set<PagePtr>,
    ret_ptr: PagePtr
)
    requires
        old_free.contains(ret_ptr),
        old_free.finite(),
        new_free =~= old_free.remove(ret_ptr),
{
    // Mutated: claim ret_ptr is still in the free set
    assert(new_free.contains(ret_ptr));
}

// Test 2: After allocation, ret.0 should be ADDED to allocated_pages_4k.
// Postcondition: allocated_pages_4k() =~= old.allocated_pages_4k().insert(ret.0).
// Mutated: assert ret.0 is NOT in the allocated set.
// SHOULD FAIL
proof fn test_mutation_page_not_allocated(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret_ptr: PagePtr
)
    requires
        !old_alloc.contains(ret_ptr),
        old_alloc.finite(),
        new_alloc =~= old_alloc.insert(ret_ptr),
{
    // Mutated: claim ret_ptr was not inserted into allocated set
    assert(!new_alloc.contains(ret_ptr));
}

// Test 3: free_pages_2m must be UNCHANGED after alloc_page_4k.
// Postcondition: free_pages_2m() =~= old.free_pages_2m().
// Mutated: assert a phantom element appeared in free_pages_2m.
// SHOULD FAIL
proof fn test_mutation_free_2m_changed(
    old_free_2m: Set<PagePtr>,
    new_free_2m: Set<PagePtr>,
    phantom: PagePtr
)
    requires
        new_free_2m =~= old_free_2m,
        !old_free_2m.contains(phantom),
{
    // Mutated: claim a new page appeared in free_pages_2m
    assert(new_free_2m.contains(phantom));
}

// Test 4: mapped_pages_4k must be UNCHANGED after alloc_page_4k.
// Postcondition: mapped_pages_4k() =~= old.mapped_pages_4k().
// Mutated: assert the returned page appeared in mapped_pages_4k.
// SHOULD FAIL
proof fn test_mutation_mapped_4k_gained(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret_ptr: PagePtr
)
    requires
        new_mapped =~= old_mapped,
        !old_mapped.contains(ret_ptr),
{
    // Mutated: claim allocation also mapped the page
    assert(new_mapped.contains(ret_ptr));
}

// Test 5: The free list length must decrease by exactly 1.
// Postcondition: self.free_pages_4k.len() == old(self).free_pages_4k.len() - 1.
// Mutated: assert the length stayed the same.
// SHOULD FAIL
proof fn test_mutation_free_list_length_unchanged(
    old_len: nat,
    new_len: nat
)
    requires
        old_len > 0,
        new_len == old_len - 1,
{
    // Mutated: claim the length did not change
    assert(new_len == old_len);
}

// Test 6: The returned page was NOT previously allocated.
// Postcondition: old(self).allocated_pages_4k().contains(ret.0) == false.
// Mutated: assume the postconditions hold, then assert ret WAS already allocated.
// SHOULD FAIL
proof fn test_mutation_ret_was_already_allocated(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret_ptr: PagePtr
)
    requires
        !old_alloc.contains(ret_ptr),
        new_alloc =~= old_alloc.insert(ret_ptr),
{
    // Mutated: claim ret_ptr was already in old allocated set
    assert(old_alloc.contains(ret_ptr));
}

}
