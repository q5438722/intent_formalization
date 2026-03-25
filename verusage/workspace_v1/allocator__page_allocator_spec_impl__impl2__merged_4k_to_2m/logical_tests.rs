use vstd::prelude::*;

fn main() {}

verus!{

pub type PagePtr = usize;

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

pub open spec fn page_index_2m_valid(i: usize) -> bool {
    &&& i % 512 == 0
    &&& 0 <= i < NUM_PAGES
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

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}

pub open spec fn spec_page_index_merge_2m_vaild(i: usize, j: usize) -> bool
    recommends
        page_index_2m_valid(i),
{
    i < j < i + 0x200
}


// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the
// specification of merged_4k_to_2m, testing whether the spec
// allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: page_ptr_valid does NOT imply page_ptr_2m_valid.
// 4k-aligned pointers are a superset of 2M-aligned pointers.
// ptr=0x1000 is 4k-valid but not 2M-valid, so this universal claim fails.
// Relevant: merged_4k_to_2m specifically requires 2m validity, not just 4k.
// SHOULD FAIL
proof fn test_logical_4k_valid_not_implies_2m_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 2: spec_page_index_truncate_2m is NOT injective.
// truncate_2m(0) == 0 and truncate_2m(1) == 0, so two different indices
// map to the same truncated value. Asserting they differ should fail.
// Relevant: merged pages use truncate_2m to find their parent page.
// SHOULD FAIL
proof fn test_logical_truncate_not_injective() {
    assert(spec_page_index_truncate_2m(0usize) != spec_page_index_truncate_2m(1usize));
}

// Test 3: page_ptr_2m_valid does NOT imply page_ptr_1g_valid.
// 2M-aligned pointers are a superset of 1G-aligned pointers.
// 0x200000 (2M) is 2m-valid but not 1g-valid.
// Relevant: merged_4k_to_2m creates 2m pages, not 1g pages.
// SHOULD FAIL
proof fn test_logical_2m_valid_not_implies_1g_valid(ptr: usize)
    requires
        page_ptr_2m_valid(ptr),
{
    assert(page_ptr_1g_valid(ptr));
}

// Test 4: A valid 2m page pointer is not unique.
// Multiple 2m-aligned pointers exist. Asserting ptr must be 0 fails.
// Relevant: merged_4k_to_2m does not determine which specific pointer is used.
// SHOULD FAIL
proof fn test_logical_2m_ptr_not_unique(ptr: usize)
    requires
        page_ptr_2m_valid(ptr),
{
    assert(ptr == 0usize);
}

// Test 5: Merging 512 4k pages reduces count by 512, not to 0.
// The spec says new_len == old_len - 512, which does NOT imply new_len == 0
// unless old_len was exactly 512. Asserting new_len == 0 is a stronger claim.
// Relevant: the spec does not guarantee the free 4k list becomes empty.
// SHOULD FAIL
proof fn test_logical_merge_does_not_empty_free4k(old_len: int, new_len: int)
    requires
        old_len >= 512,
        new_len == old_len - 512,
{
    assert(new_len == 0);
}

}
