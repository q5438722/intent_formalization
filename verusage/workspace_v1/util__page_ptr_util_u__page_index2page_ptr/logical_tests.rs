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

// ============ Logical Tests ============

// Test 1: NOT all valid indices produce positive pointers (i=0 maps to 0)
// SHOULD FAIL
proof fn logical_always_positive() {
    assert(forall|i: usize| page_index_valid(i) ==> spec_page_index2page_ptr(i) > 0usize);
}

// Test 2: Result is NOT always >= 4096 (i=0 maps to 0)
// SHOULD FAIL
proof fn logical_minimum_4096() {
    assert(spec_page_index2page_ptr(0usize) >= 4096usize);
}

// Test 3: Result NOT always less than NUM_PAGES (index 512 maps to exactly NUM_PAGES)
// SHOULD FAIL
proof fn logical_result_bounded_by_num_pages() {
    assert(spec_page_index2page_ptr(512usize) < NUM_PAGES);
}

// Test 4: Strict monotonicity gap — result(1) is NOT strictly greater than result(0) + 4096
// SHOULD FAIL
proof fn logical_stronger_monotonicity() {
    assert(spec_page_index2page_ptr(1usize) > add(spec_page_index2page_ptr(0usize), 4096usize));
}

// Test 5: Result is NOT always odd — 4096 is even
// SHOULD FAIL
proof fn logical_result_always_odd() {
    assert(spec_page_index2page_ptr(1usize) % 2 == 1usize);
}

// Helper spec to avoid overflow concerns in proof context
pub open spec fn add(a: usize, b: usize) -> usize {
    (a + b) as usize
}

}
