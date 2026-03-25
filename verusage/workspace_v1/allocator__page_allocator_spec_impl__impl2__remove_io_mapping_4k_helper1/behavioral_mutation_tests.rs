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

// Test 1: After removing an IO mapping, the removed pair must NOT still be present.
// Postcondition: page_io_mappings(target) =~= old.page_io_mappings(target).remove((ioid, va)).
// Mutated: assert the IO mapping IS still present after remove.
// SHOULD FAIL
proof fn test_mutation_io_mapping_still_present(
    old_io_mappings: Set<(IOid, VAddr)>,
    new_io_mappings: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        old_io_mappings.contains((ioid, va)),
        old_io_mappings.finite(),
        old_io_mappings.len() == 1,
        new_io_mappings =~= old_io_mappings.remove((ioid, va)),
{
    // Mutated: claim the IO mapping survived removal
    assert(new_io_mappings.contains((ioid, va)));
}

// Test 2: Regular mappings must remain unchanged after remove_io_mapping_4k_helper1.
// Postcondition: page_mappings(target) =~= old.page_mappings(target).
// Mutated: assert a phantom element appeared in page_mappings.
// SHOULD FAIL
proof fn test_mutation_page_mappings_gained(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    x: (Pcid, VAddr)
)
    requires
        new_mappings =~= old_mappings,
        !old_mappings.contains(x),
{
    // Mutated: claim a new regular mapping appeared
    assert(new_mappings.contains(x));
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
    // Mutated: claim a new page was added to allocated_pages_4k
    assert(new_alloc.contains(p));
}

// Test 4: Other mapped pages' IO mappings must be preserved.
// Postcondition: for p != target_ptr, page_io_mappings(p) =~= old.page_io_mappings(p).
// Mutated: assert another page's IO mappings changed.
// SHOULD FAIL
proof fn test_mutation_other_page_io_mappings_changed(
    old_other_io: Set<(IOid, VAddr)>,
    new_other_io: Set<(IOid, VAddr)>,
    x: (IOid, VAddr)
)
    requires
        new_other_io =~= old_other_io,
        old_other_io.contains(x),
{
    // Mutated: claim the other page lost an IO mapping
    assert(!new_other_io.contains(x));
}

// Test 5: container_map_4k must be updated to remove target_ptr from
// the owning container's page set.
// Mutated: assert the target_ptr is still in the container's set after update.
// SHOULD FAIL
proof fn test_mutation_container_map_4k_not_updated(
    old_container_set: Set<PagePtr>,
    new_container_set: Set<PagePtr>,
    target_ptr: PagePtr
)
    requires
        old_container_set.contains(target_ptr),
        old_container_set.finite(),
        new_container_set =~= old_container_set.remove(target_ptr),
{
    // Mutated: claim target_ptr is still in the container's page set
    assert(new_container_set.contains(target_ptr));
}

// Test 6: container_map_2m must be unchanged.
// Postcondition: container_map_2m =~= old.container_map_2m.
// Mutated: assert container_map_2m differs.
// SHOULD FAIL
proof fn test_mutation_container_map_2m_changed(
    old_map: Map<ContainerPtr, Set<PagePtr>>,
    new_map: Map<ContainerPtr, Set<PagePtr>>,
    c: ContainerPtr,
    s: Set<PagePtr>
)
    requires
        new_map =~= old_map,
        !old_map.dom().contains(c),
{
    // Mutated: claim a new container appeared in container_map_2m
    assert(new_map.dom().contains(c));
}

}
