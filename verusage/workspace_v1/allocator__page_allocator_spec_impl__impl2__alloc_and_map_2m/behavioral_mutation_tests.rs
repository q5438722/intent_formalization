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

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test models a postcondition of alloc_and_map_2m, then mutates the
// expected output to assert an incorrect behavior.
// All tests SHOULD FAIL verification.

// Test 1: free_pages_2m() =~= old.free_pages_2m().remove(ret).
// Mutated: assert ret is STILL in the free set after removal.
// SHOULD FAIL
proof fn test_mutation_ret_still_in_free_2m(
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

// Test 2: mapped_pages_2m() =~= old.mapped_pages_2m().insert(ret).
// Mutated: assert mapped_pages_2m is unchanged (ret NOT added).
// SHOULD FAIL
proof fn test_mutation_mapped_2m_unchanged(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_mapped.contains(ret),
        new_mapped =~= old_mapped.insert(ret),
{
    // ret should be in new_mapped, but we assert otherwise
    assert(!new_mapped.contains(ret));
}

// Test 3: page_mappings(ret) =~= Set::empty().insert((pcid, va)).
// Mutated: assert page_mappings(ret) is empty after alloc_and_map_2m.
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
// Mutated: assert io_mappings is non-empty after alloc_and_map_2m.
// SHOULD FAIL
proof fn test_mutation_io_mappings_nonempty(
    io_mappings: Set<(IOid, VAddr)>,
    x: (IOid, VAddr),
)
    requires
        io_mappings =~= Set::<(IOid, VAddr)>::empty(),
{
    // io_mappings should be empty, asserting it contains x should fail
    assert(io_mappings.contains(x));
}

// Test 5: free_pages_4k() =~= old.free_pages_4k() (4k pages unchanged by 2m alloc).
// Mutated: assert 4k free pages lost an element after a 2m alloc.
// SHOULD FAIL
proof fn test_mutation_4k_free_changed(
    old_free_4k: Set<PagePtr>,
    new_free_4k: Set<PagePtr>,
    p: PagePtr,
)
    requires
        new_free_4k =~= old_free_4k,
        old_free_4k.contains(p),
{
    // 4k free set is unchanged by 2m alloc, so p should still be there
    assert(!new_free_4k.contains(p));
}

// Test 6: allocated_pages_2m() =~= old.allocated_pages_2m() (2m allocated unchanged).
// Mutated: assert ret was added to allocated_pages_2m (wrong: it goes to mapped, not allocated).
// SHOULD FAIL
proof fn test_mutation_ret_in_allocated_2m(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_alloc.contains(ret),
        new_alloc =~= old_alloc,
{
    // allocated_pages_2m is unchanged, ret should not be in it
    assert(new_alloc.contains(ret));
}

// Test 7: Other mapped pages preserve their mappings.
// Mutated: assert another page's mapping changed after alloc_and_map_2m.
// SHOULD FAIL
proof fn test_mutation_other_page_mapping_changed(
    old_mapping: Set<(Pcid, VAddr)>,
    new_mapping: Set<(Pcid, VAddr)>,
    extra: (Pcid, VAddr),
)
    requires
        new_mapping =~= old_mapping,
        !old_mapping.contains(extra),
{
    // Other pages' mappings are preserved, extra should not appear
    assert(new_mapping.contains(extra));
}

}
