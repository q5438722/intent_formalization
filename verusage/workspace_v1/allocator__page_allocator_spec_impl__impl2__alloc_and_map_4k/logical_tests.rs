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
// Each test asserts a property NOT explicitly guaranteed by alloc_and_map_4k,
// testing whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: Determinism - the spec does not guarantee which free page is returned.
// Asserting two different free sets yield the same return violates nondeterminism.
// SHOULD FAIL
proof fn test_logical_determinism(
    free_set: Set<PagePtr>,
    ret1: PagePtr,
    ret2: PagePtr,
)
    requires
        free_set.contains(ret1),
        free_set.contains(ret2),
        page_ptr_valid(ret1),
        page_ptr_valid(ret2),
        ret1 != ret2,
{
    // Both are valid returns from the same free set; they need not be equal
    assert(ret1 == ret2);
}

// Test 2: Stronger inequality - free_pages_4k.len() decreases by exactly 1, not 2.
// SHOULD FAIL
proof fn test_logical_free_count_decreased_by_two(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 0,
        new_len == old_len - 1,
{
    assert(new_len == old_len - 2);
}

// Test 3: page_ptr_valid does NOT imply page_ptr_2m_valid.
// alloc_and_map_4k guarantees page_ptr_valid(ret), but not page_ptr_2m_valid(ret).
// SHOULD FAIL
proof fn test_logical_4k_valid_implies_2m_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 4: The spec does not guarantee ret equals any specific value.
// Asserting ret must be 0 is a structural assumption not in the spec.
// SHOULD FAIL
proof fn test_logical_ret_is_zero(
    ret: PagePtr,
)
    requires
        page_ptr_valid(ret),
{
    assert(ret == 0usize);
}

// Test 5: Inserting ret into mapped_pages and removing from free_pages
// does NOT mean the total count (free + mapped) changes by 2.
// The total should be unchanged (one moves from free to mapped).
// Asserting total decreased by 2 is a wrong global invariant.
// SHOULD FAIL
proof fn test_logical_total_pages_decreased(
    old_free_len: int,
    new_free_len: int,
    old_mapped_len: int,
    new_mapped_len: int,
)
    requires
        old_free_len > 0,
        new_free_len == old_free_len - 1,
        new_mapped_len == old_mapped_len + 1,
{
    // Total should be conserved: (old_free + old_mapped) == (new_free + new_mapped)
    // Asserting total decreased by 2 is wrong
    assert(new_free_len + new_mapped_len == old_free_len + old_mapped_len - 2);
}

// Test 6: Cross-function misuse - alloc_and_map_4k maps into 4k pages.
// The ret should NOT appear in mapped_pages_2m.
// Asserting ret is in mapped_pages_2m after a 4k alloc is a category error.
// SHOULD FAIL
proof fn test_logical_ret_in_wrong_category(
    old_mapped_2m: Set<PagePtr>,
    new_mapped_2m: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        new_mapped_2m =~= old_mapped_2m,
        !old_mapped_2m.contains(ret),
{
    // mapped_pages_2m is unchanged by alloc_and_map_4k, so ret not in it
    assert(new_mapped_2m.contains(ret));
}

// Test 7: The mapping set for ret has exactly 1 element.
// Asserting it has 2 elements is a stronger property not guaranteed.
// SHOULD FAIL
proof fn test_logical_mapping_has_two_elements(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid,
    va: VAddr,
)
    requires
        mappings =~= Set::<(Pcid, VAddr)>::empty().insert((pcid, va)),
        mappings.finite(),
{
    assert(mappings.len() == 2);
}

}
