use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type PagePtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
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
// Each test asserts a property NOT explicitly guaranteed by the specification,
// testing whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_valid does not uniquely identify a single pointer.
// Many valid pointers exist. Asserting ptr must be 0 should fail.
// SHOULD FAIL
proof fn test_logical_valid_ptr_not_unique(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr == 0usize);
}

// Test 2: Adding one mapping increases count by exactly 1, NOT 2.
// The spec guarantees len increases by 1. Asserting +2 is a stronger inequality.
// SHOULD FAIL
proof fn test_logical_mapping_count_up_by_two(
    old_mappings: Set<(Pcid, VAddr)>,
    new_mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        old_mappings.finite(),
        !old_mappings.contains((pcid, va)),
        new_mappings =~= old_mappings.insert((pcid, va)),
{
    assert(new_mappings.len() == old_mappings.len() + 2);
}

// Test 3: page_index2page_ptr and page_ptr2page_index form a valid roundtrip.
// Assert the roundtrip BREAKS for the concrete value 4096.
// spec_page_ptr2page_index(4096) = 1, spec_page_index2page_ptr(1) = 4096.
// SHOULD FAIL
proof fn test_logical_roundtrip_breaks() {
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(4096usize)) != 4096usize);
}

// Test 4: page_ptr_valid does NOT imply page_ptr_2m_valid.
// 4k-aligned pointers are a superset of 2M-aligned pointers.
// ptr=0x1000 is 4k-valid but not 2M-valid, so this universal claim fails.
// SHOULD FAIL
proof fn test_logical_4k_valid_implies_2m_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 5: Inserting into a non-empty set does NOT make it empty.
// The set can only grow or stay the same size after insert.
// SHOULD FAIL
proof fn test_logical_insert_empties_set(
    s: Set<(Pcid, VAddr)>,
    pcid: Pcid, va: VAddr
)
    requires
        s.finite(),
        s.len() > 0,
{
    let new_s = s.insert((pcid, va));
    assert(new_s.len() == 0);
}

}
