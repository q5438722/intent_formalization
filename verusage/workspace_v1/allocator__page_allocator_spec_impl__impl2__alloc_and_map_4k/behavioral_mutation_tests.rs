use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type PagePtr = usize;
pub type ContainerPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test models a postcondition of alloc_and_map_4k, then mutates the
// expected output to assert an incorrect behavior.
// All tests SHOULD FAIL verification.

// Test 1: free_pages_4k() =~= old.free_pages_4k().remove(ret).
// Mutated: assert ret is STILL in the free set after removal.
// SHOULD FAIL
proof fn test_mutation_ret_still_free(
    old_free: Set<PagePtr>,
    new_free: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        old_free.contains(ret),
        new_free =~= old_free.remove(ret),
{
    // ret was removed, so it should NOT be in new_free
    assert(new_free.contains(ret));
}

// Test 2: mapped_pages_4k() =~= old.mapped_pages_4k().insert(ret).
// Mutated: assert mapped_pages_4k is unchanged (ret NOT added).
// SHOULD FAIL
proof fn test_mutation_mapped_unchanged(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_mapped.contains(ret),
        new_mapped =~= old_mapped.insert(ret),
{
    // ret should now be in new_mapped, but we assert otherwise
    assert(!new_mapped.contains(ret));
}

// Test 3: page_mappings(ret) =~= Set::empty().insert((pcid, va)).
// Mutated: assert page_mappings(ret) is empty after alloc_and_map.
// SHOULD FAIL
proof fn test_mutation_mapping_empty(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid,
    va: VAddr,
)
    requires
        mappings =~= Set::<(Pcid, VAddr)>::empty().insert((pcid, va)),
{
    // The mapping should contain (pcid, va), not be empty
    assert(mappings =~= Set::<(Pcid, VAddr)>::empty());
}

// Test 4: page_io_mappings(ret) =~= Set::empty().
// Mutated: assert io_mappings is non-empty after alloc_and_map.
// SHOULD FAIL
proof fn test_mutation_io_mappings_nonempty(
    io_mappings: Set<(IOid, VAddr)>,
    x: (IOid, VAddr),
)
    requires
        io_mappings =~= Set::<(IOid, VAddr)>::empty(),
{
    // io_mappings should be empty, but we assert it contains an element
    assert(io_mappings.contains(x));
}

// Test 5: free_pages_2m() =~= old.free_pages_2m() (2m pages unchanged).
// Mutated: assert 2m free pages lost an element after a 4k alloc.
// SHOULD FAIL
proof fn test_mutation_2m_free_changed(
    old_free_2m: Set<PagePtr>,
    new_free_2m: Set<PagePtr>,
    p: PagePtr,
)
    requires
        new_free_2m =~= old_free_2m,
        old_free_2m.contains(p),
{
    // 2m free set is unchanged, so p should still be there
    assert(!new_free_2m.contains(p));
}

// Test 6: get_container_owned_pages(c_ptr) =~= old.get_container_owned_pages(c_ptr).insert(ret).
// Mutated: assert the old pages disappeared (container lost all previous pages).
// SHOULD FAIL
proof fn test_mutation_container_pages_lost(
    old_pages: Set<PagePtr>,
    new_pages: Set<PagePtr>,
    ret: PagePtr,
    existing_page: PagePtr,
)
    requires
        old_pages.contains(existing_page),
        existing_page != ret,
        new_pages =~= old_pages.insert(ret),
{
    // existing_page was in old_pages and should be preserved after insert
    assert(!new_pages.contains(existing_page));
}

// Test 7: old(self).allocated_pages_4k().contains(ret) == false.
// Mutated: assert ret WAS allocated before the call.
// SHOULD FAIL
proof fn test_mutation_ret_was_allocated(
    allocated: Set<PagePtr>,
    free_set: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        free_set.contains(ret),
        allocated.finite(),
        free_set.finite(),
        forall|p: PagePtr| free_set.contains(p) ==> !allocated.contains(p),
{
    // ret is in free_set, which is disjoint from allocated
    assert(allocated.contains(ret));
}

}
