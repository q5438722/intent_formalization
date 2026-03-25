use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type PagePtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates the expected output/relation.
// These test whether the spec rejects incorrect behaviors.
// All tests SHOULD FAIL verification.

// Test 1: After inserting a mapping, the set MUST contain it.
// Mutated assertion: the new mapping is NOT in the result.
// Models: add_mapping_4k postcondition page_mappings(target) =~= old.insert((pcid,va)).
// SHOULD FAIL
proof fn test_mutation_mapping_not_inserted(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        !old_mappings.contains((pcid, va)),
        new_mappings =~= old_mappings.insert((pcid, va)),
{
    assert(!new_mappings.contains((pcid, va)));
}

// Test 2: After inserting a new mapping, previously existing mappings are preserved.
// Mutated assertion: an old mapping was removed by the insert.
// Models: add_mapping_4k postcondition that other mappings are preserved.
// SHOULD FAIL
proof fn test_mutation_old_mapping_removed(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr,
    other_pcid: Pcid, other_va: VAddr
)
    requires
        old_mappings.contains((other_pcid, other_va)),
        (other_pcid, other_va) !== (pcid, va),
        new_mappings =~= old_mappings.insert((pcid, va)),
{
    assert(!new_mappings.contains((other_pcid, other_va)));
}

// Test 3: io_mappings are unchanged after add_mapping_4k.
// Mutated assertion: a new element appeared in io_mappings.
// Models: postcondition io_mappings(target) =~= old.io_mappings(target).
// SHOULD FAIL
proof fn test_mutation_io_mappings_gained_element(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>,
    x: (IOid, VAddr)
)
    requires
        new_io =~= old_io,
        !old_io.contains(x),
{
    assert(new_io.contains(x));
}

// Test 4: mapped_pages_4k is unchanged after add_mapping_4k.
// Mutated assertion: a new page appeared in mapped_pages_4k.
// Models: postcondition mapped_pages_4k() =~= old.mapped_pages_4k().
// SHOULD FAIL
proof fn test_mutation_mapped_pages_gained_page(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    p: PagePtr
)
    requires
        new_mapped =~= old_mapped,
        !old_mapped.contains(p),
{
    assert(new_mapped.contains(p));
}

// Test 5: free_pages_4k is unchanged after add_mapping_4k.
// Mutated assertion: a page was freed (added to free set).
// Models: postcondition free_pages_4k() =~= old.free_pages_4k().
// SHOULD FAIL
proof fn test_mutation_free_pages_gained_page(
    old_free: Set<PagePtr>,
    new_free: Set<PagePtr>,
    p: PagePtr
)
    requires
        new_free =~= old_free,
        !old_free.contains(p),
{
    assert(new_free.contains(p));
}

}
