use vstd::prelude::*;

fn main() {}

verus!{

pub type PagePtr = usize;
pub type Pcid = usize;
pub type VAddr = usize;

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
// Many valid pointers exist (0, 0x1000, 0x2000, ...).
// Asserting ptr must equal 0 when valid is a false determinism claim.
// SHOULD FAIL
proof fn test_logical_valid_ptr_not_unique(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr == 0usize);
}

// Test 2: Removing one element from a multi-element set does NOT empty it.
// The spec removes target_ptr from allocated_pages_4k. This should NOT make
// the allocated set empty when other pages are allocated.
// Asserting the set is empty after remove is a false global claim.
// SHOULD FAIL
proof fn test_logical_remove_empties_set(
    s: Set<PagePtr>,
    target: PagePtr,
    other: PagePtr,
)
    requires
        s.finite(),
        s.contains(target),
        s.contains(other),
        target != other,
{
    let new_s = s.remove(target);
    assert(new_s.len() == 0);
}

// Test 3: page_ptr_valid does NOT imply page_ptr_2m_valid.
// 4k-aligned pointers are a superset of 2M-aligned pointers.
// This is a false over-generalization: free_page_4k deals with 4k pages,
// which are NOT necessarily 2m-valid.
// SHOULD FAIL
proof fn test_logical_4k_valid_implies_2m_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 4: The roundtrip page_index2page_ptr(page_ptr2page_index(x)) should NOT break.
// Asserting it breaks for a known valid value (4096) tests a false negation.
// spec_page_ptr2page_index(4096) = 1, spec_page_index2page_ptr(1) = 4096.
// SHOULD FAIL
proof fn test_logical_roundtrip_breaks() {
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(4096usize)) != 4096usize);
}

// Test 5: Inserting into a non-empty set cannot make it empty.
// After free_page_4k, target_ptr is inserted into free_pages_4k.
// The resulting set should have at least 1 element.
// Asserting it has 0 elements is a false structural claim.
// SHOULD FAIL
proof fn test_logical_insert_empties_set(
    s: Set<PagePtr>,
    target: PagePtr,
)
    requires
        s.finite(),
{
    let new_s = s.insert(target);
    assert(new_s.len() == 0);
}

}
