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
// Each test models a postcondition of alloc_and_map_io_4k, then mutates
// the expected output to assert an incorrect behavior.
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
    assert(new_free.contains(ret));
}

// Test 2: mapped_pages_4k() =~= old.mapped_pages_4k().insert(ret).
// Mutated: assert ret is NOT in the new mapped set.
// SHOULD FAIL
proof fn test_mutation_ret_not_mapped(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_mapped.contains(ret),
        new_mapped =~= old_mapped.insert(ret),
{
    assert(!new_mapped.contains(ret));
}

// Test 3: page_io_mappings(ret) =~= Set::empty().insert((ioid, va)).
// Mutated: assert io_mappings is empty after alloc_and_map_io.
// SHOULD FAIL
proof fn test_mutation_io_mappings_empty(
    io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid,
    va: VAddr,
)
    requires
        io_mappings =~= Set::<(IOid, VAddr)>::empty().insert((ioid, va)),
{
    assert(io_mappings =~= Set::<(IOid, VAddr)>::empty());
}

// Test 4: page_mappings(ret) =~= Set::empty().
// Mutated: assert page_mappings is non-empty (has some arbitrary mapping).
// SHOULD FAIL
proof fn test_mutation_regular_mappings_nonempty(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid,
    va: VAddr,
)
    requires
        mappings =~= Set::<(Pcid, VAddr)>::empty(),
{
    assert(mappings.contains((pcid, va)));
}

// Test 5: free_pages_4k.len() == old.free_pages_4k.len() - 1.
// Mutated: assert the free count stays the same.
// SHOULD FAIL
proof fn test_mutation_free_count_unchanged(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 0,
        new_len == old_len - 1,
{
    assert(new_len == old_len);
}

// Test 6: allocated_pages_4k() =~= old.allocated_pages_4k() (unchanged).
// Mutated: assert allocated_pages_4k now contains ret.
// SHOULD FAIL
proof fn test_mutation_allocated_contains_ret(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_alloc.contains(ret),
        new_alloc =~= old_alloc,
{
    assert(new_alloc.contains(ret));
}

// Test 7: get_container_owned_pages(c_ptr) =~= old.get_container_owned_pages(c_ptr).insert(ret).
// Mutated: assert container owned pages for c_ptr are unchanged (ret not added).
// SHOULD FAIL
proof fn test_mutation_container_pages_unchanged(
    old_owned: Set<PagePtr>,
    new_owned: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        !old_owned.contains(ret),
        new_owned =~= old_owned.insert(ret),
{
    assert(new_owned =~= old_owned);
}

// Test 8: !old(self).page_is_mapped(ret) (ret was free, not mapped).
// Mutated: assert ret was already mapped before alloc.
// SHOULD FAIL
proof fn test_mutation_ret_was_already_mapped(
    was_mapped: bool,
)
    requires
        was_mapped == false,
{
    assert(was_mapped == true);
}

}
