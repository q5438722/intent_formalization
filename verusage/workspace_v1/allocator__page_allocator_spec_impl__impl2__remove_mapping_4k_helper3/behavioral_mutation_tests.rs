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

// Test 1: After removing a mapping, the removed pair must NOT still be present.
// Postcondition: page_mappings(target) =~= old.page_mappings(target).remove((pcid,va)).
// Mutated: assert the pair IS still present after remove.
// SHOULD FAIL
proof fn test_mutation_mapping_still_present(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        old_mappings.contains((pcid, va)),
        old_mappings.finite(),
        old_mappings.len() == 3,
        new_mappings =~= old_mappings.remove((pcid, va)),
{
    // Mutated: claim the mapping survived removal
    assert(new_mappings.contains((pcid, va)));
}

// Test 2: After removing a mapping, other existing mappings must be preserved.
// Mutated: assert a different mapping was also removed.
// SHOULD FAIL
proof fn test_mutation_other_mapping_lost(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr,
    other_pcid: Pcid, other_va: VAddr
)
    requires
        old_mappings.contains((pcid, va)),
        old_mappings.contains((other_pcid, other_va)),
        (other_pcid, other_va) !== (pcid, va),
        old_mappings.finite(),
        new_mappings =~= old_mappings.remove((pcid, va)),
{
    // Mutated: claim other mapping was also removed
    assert(!new_mappings.contains((other_pcid, other_va)));
}

// Test 3: io_mappings must remain unchanged after remove_mapping_4k_helper3.
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

// Test 4: allocated_pages_4k must be unchanged after the operation.
// Postcondition: allocated_pages_4k() =~= old.allocated_pages_4k().
// Mutated: assert a new page appeared in allocated_pages_4k.
// SHOULD FAIL
proof fn test_mutation_allocated_pages_gained(
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

// Test 5: container_map_4k must be unchanged after remove_mapping_4k_helper3.
// Unlike helper1, helper3 preserves container_map_4k entirely because
// the page stays mapped (ref_count != 1).
// Mutated: assert a page was removed from a container set.
// SHOULD FAIL
proof fn test_mutation_container_map_changed(
    old_cmap: Map<ContainerPtr, Set<PagePtr>>,
    new_cmap: Map<ContainerPtr, Set<PagePtr>>,
    c: ContainerPtr,
    target_ptr: PagePtr
)
    requires
        new_cmap =~= old_cmap,
        old_cmap.dom().contains(c),
        old_cmap[c].contains(target_ptr),
{
    // Mutated: claim target_ptr was removed from container map
    assert(!new_cmap[c].contains(target_ptr));
}

}
