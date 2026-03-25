use vstd::prelude::*;

fn main() {}

verus!{

pub type VAddr = usize;
pub type PagePtr = usize;
pub type IOid = usize;
pub type Pcid = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates the expected output/relation.
// These test whether the spec rejects incorrect behaviors for add_io_mapping_4k.
// All tests SHOULD FAIL verification.

// Test 1: After inserting an IO mapping, the set MUST contain the new pair.
// Mutated: assert the new IO mapping is NOT present.
// Models: postcondition page_io_mappings(target) =~= old.insert((ioid, va)).
// SHOULD FAIL
proof fn test_mutation_io_mapping_not_inserted(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        !old_io.contains((ioid, va)),
        new_io =~= old_io.insert((ioid, va)),
{
    assert(!new_io.contains((ioid, va)));
}

// Test 2: After inserting a new IO mapping, previously existing IO mappings
// must be preserved. Mutated: assert an old IO mapping was removed.
// Models: postcondition that io_mappings only gains the new pair.
// SHOULD FAIL
proof fn test_mutation_old_io_mapping_removed(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr,
    other_ioid: IOid, other_va: VAddr
)
    requires
        old_io.contains((other_ioid, other_va)),
        (other_ioid, other_va) !== (ioid, va),
        new_io =~= old_io.insert((ioid, va)),
{
    assert(!new_io.contains((other_ioid, other_va)));
}

// Test 3: add_io_mapping_4k preserves page_mappings (regular mappings).
// Mutated: assert that a new element appeared in page_mappings.
// Models: postcondition page_mappings(target) =~= old.page_mappings(target).
// SHOULD FAIL
proof fn test_mutation_page_mappings_gained_element(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    x: (Pcid, VAddr)
)
    requires
        new_mappings =~= old_mappings,
        !old_mappings.contains(x),
{
    assert(new_mappings.contains(x));
}

// Test 4: add_io_mapping_4k preserves mapped_pages_4k.
// Mutated: assert a new page appeared in mapped_pages_4k.
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

// Test 5: add_io_mapping_4k preserves free_pages_4k.
// Mutated: assert a page was freed (added to free set).
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
