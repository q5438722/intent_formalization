use vstd::prelude::*;

fn main() {}

verus!{

// ============ Original Definitions ============

#[verifier::external_body]
#[verifier(external_body)]
pub proof fn lemma_usize_u64(x: u64)
    ensures
        x as usize as u64 == x,
{
    unimplemented!()
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}

#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires
        0 <= i < NUM_PAGES,
    ensures
        ret == spec_page_index2page_ptr(i),
{
    proof {
        lemma_usize_u64(MAX_USIZE);
    }
    i * 4096usize
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub const MAX_USIZE: u64 = 31 * 1024 * 1024 * 1024;

// ============ Boundary Tests ============

// Test 1: NUM_PAGES is NOT a valid page index (upper boundary exclusive)
// SHOULD FAIL
proof fn boundary_test_at_limit() {
    assert(page_index_valid(NUM_PAGES));
}

// Test 2: One past NUM_PAGES is NOT valid (2097153 = NUM_PAGES + 1)
// SHOULD FAIL
proof fn boundary_test_beyond_limit() {
    assert(page_index_valid(2097153usize));
}

// Test 3: A much larger value is NOT a valid page index (4194304 = 4 * 1024 * 1024)
// SHOULD FAIL
proof fn boundary_test_large_value() {
    assert(page_index_valid(4194304usize));
}

// Test 4: Edge case — index 0 maps to pointer 0, not a positive value
// SHOULD FAIL
proof fn boundary_test_zero_index_nonzero_result() {
    assert(spec_page_index2page_ptr(0usize) != 0usize);
}

}
