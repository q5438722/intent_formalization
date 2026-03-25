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


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs (matching alloc_page_2m postconditions)
// and mutates expected outputs/relations.
// All tests SHOULD FAIL verification.

// Test 1: After allocation, the returned page should be REMOVED from free_pages_2m.
// Postcondition: free_pages_2m() =~= old.free_pages_2m().remove(ret.0).
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
    // Mutated: claim ret_ptr is still in free set
    assert(new_free.contains(ret_ptr));
}

// Test 2: After allocation, ret.0 should be ADDED to allocated_pages_2m.
// Postcondition: allocated_pages_2m() =~= old.allocated_pages_2m().insert(ret.0).
// Mutated: assert ret.0 is NOT in allocated set.
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
    // Mutated: claim ret_ptr was not inserted
    assert(!new_alloc.contains(ret_ptr));
}

// Test 3: free_pages_4k must be UNCHANGED after alloc_page_2m.
// Postcondition: free_pages_4k() =~= old.free_pages_4k().
// Mutated: assert a phantom element appeared in free_pages_4k.
// SHOULD FAIL
proof fn test_mutation_free_4k_changed(
    old_free_4k: Set<PagePtr>,
    new_free_4k: Set<PagePtr>,
    phantom: PagePtr
)
    requires
        new_free_4k =~= old_free_4k,
        !old_free_4k.contains(phantom),
{
    // Mutated: claim a new page appeared in free_pages_4k
    assert(new_free_4k.contains(phantom));
}

// Test 4: mapped_pages_2m must be UNCHANGED after alloc_page_2m.
// Postcondition: mapped_pages_2m() =~= old.mapped_pages_2m().
// Mutated: assert the returned page appeared in mapped_pages_2m.
// SHOULD FAIL
proof fn test_mutation_mapped_2m_gained(
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

// Test 5: allocated_pages_1g must be UNCHANGED after alloc_page_2m.
// Postcondition: allocated_pages_1g() =~= old.allocated_pages_1g().
// Mutated: assert a new page appeared in allocated_pages_1g.
// SHOULD FAIL
proof fn test_mutation_allocated_1g_changed(
    old_alloc_1g: Set<PagePtr>,
    new_alloc_1g: Set<PagePtr>,
    phantom: PagePtr
)
    requires
        new_alloc_1g =~= old_alloc_1g,
        !old_alloc_1g.contains(phantom),
{
    // Mutated: claim a 1g page was also allocated
    assert(new_alloc_1g.contains(phantom));
}

}
