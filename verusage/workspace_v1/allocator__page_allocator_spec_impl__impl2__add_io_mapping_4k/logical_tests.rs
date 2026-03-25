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

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}


// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the
// add_io_mapping_4k specification, testing whether the spec
// allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_valid does not uniquely determine a pointer.
// Many valid pointers exist, so constraining ptr == 0 is too strong.
// SHOULD FAIL
proof fn test_logical_valid_ptr_not_unique(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr == 0usize);
}

// Test 2: After inserting one IO mapping, the count increases by 1,
// NOT by 2. Asserting +2 is a stronger inequality not entailed.
// Models: postcondition io_mappings.len() == old.len() + 1.
// SHOULD FAIL
proof fn test_logical_io_mapping_count_up_by_two(
    old_io: Set<(IOid, VAddr)>,
    new_io: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        old_io.finite(),
        !old_io.contains((ioid, va)),
        new_io =~= old_io.insert((ioid, va)),
{
    assert(new_io.len() == old_io.len() + 2);
}

// Test 3: page_index/page_ptr roundtrip should hold for valid values.
// Assert it BREAKS for the concrete value 4096 — this should fail
// since spec_page_ptr2page_index(4096)=1 and spec_page_index2page_ptr(1)=4096.
// SHOULD FAIL
proof fn test_logical_roundtrip_breaks() {
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(4096usize)) != 4096usize);
}

// Test 4: page_ptr_valid does NOT imply page_ptr_1g_valid.
// 4k-aligned pointers are a superset of 1G-aligned pointers.
// ptr=0x1000 is 4k-valid but not 1G-valid.
// SHOULD FAIL
proof fn test_logical_4k_valid_implies_1g_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_1g_valid(ptr));
}

// Test 5: Inserting into a non-empty set does NOT make it empty.
// The set can only grow or stay after insert.
// SHOULD FAIL
proof fn test_logical_io_insert_empties_set(
    s: Set<(IOid, VAddr)>,
    ioid: IOid, va: VAddr
)
    requires
        s.finite(),
        s.len() > 0,
{
    let new_s = s.insert((ioid, va));
    assert(new_s.len() == 0);
}

}
