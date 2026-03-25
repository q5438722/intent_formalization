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
// Each test starts from valid inputs and mutates the expected output/relation.
// These test whether the spec rejects incorrect behaviors.
// All tests SHOULD FAIL verification.

// Test 1: After removing a mapping, the removed pair must NOT be present.
// Postcondition: page_mappings(target) =~= old.page_mappings(target).remove((pcid,va)).
// Mutated: assert the pair IS still present after removal.
// SHOULD FAIL
proof fn test_mutation_mapping_still_present(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        old_mappings.contains((pcid, va)),
        old_mappings.finite(),
        old_mappings.len() == 1,
        new_mappings =~= old_mappings.remove((pcid, va)),
{
    // Mutated: claim the mapping survived removal
    assert(new_mappings.contains((pcid, va)));
}

// Test 2: io_mappings must remain unchanged after remove_mapping_4k_helper2.
// Postcondition: page_io_mappings(target) =~= old.page_io_mappings(target).
// Mutated: assert a phantom element appeared in io_mappings.
// SHOULD FAIL
proof fn test_mutation_io_mappings_gained(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>,
    x: (IOid, VAddr)
)
    requires
        new_io =~= old_io,
        !old_io.contains(x),
{
    // Mutated: claim a new element appeared
    assert(new_io.contains(x));
}

// Test 3: allocated_pages_4k must be unchanged after the operation.
// Postcondition: allocated_pages_4k() =~= old.allocated_pages_4k().
// Mutated: assert a new page appeared in allocated_pages_4k.
// SHOULD FAIL
proof fn test_mutation_allocated_pages_4k_gained(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    p: PagePtr
)
    requires
        new_alloc =~= old_alloc,
        !old_alloc.contains(p),
{
    // Mutated: claim a new page was allocated
    assert(new_alloc.contains(p));
}

// Test 4: container_map_4k must be updated to remove target_ptr from
// the owning container's set. Mutated: assert target_ptr is still
// in the container's page set.
// SHOULD FAIL
proof fn test_mutation_container_map_4k_target_not_removed(
    old_container_set: Set<PagePtr>,
    new_container_set: Set<PagePtr>,
    target_ptr: PagePtr
)
    requires
        old_container_set.contains(target_ptr),
        old_container_set.finite(),
        new_container_set =~= old_container_set.remove(target_ptr),
{
    // Mutated: claim target is still in the container set
    assert(new_container_set.contains(target_ptr));
}

// Test 5: container_map_2m must be unchanged after the operation.
// Postcondition: container_map_2m =~= old.container_map_2m.
// Mutated: assert the maps differ (new_map lost a key).
// SHOULD FAIL
proof fn test_mutation_container_map_2m_changed(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    new_map: Map<ContainerPtr, Set<PagePtr>>,
    c: ContainerPtr
)
    requires
        new_map =~= old_map,
        old_map.dom().contains(c),
{
    // Mutated: claim the key disappeared from the new map
    assert(!new_map.dom().contains(c));
}

}
