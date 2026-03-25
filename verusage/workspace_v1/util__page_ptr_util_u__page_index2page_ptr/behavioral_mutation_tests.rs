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

// ============ Behavioral Mutation Tests ============

// Test 1: Wrong multiplier — 4095 instead of 4096
// SHOULD FAIL
proof fn mutation_wrong_multiplier() {
    assert(spec_page_index2page_ptr(1usize) == 4095usize);
}

// Test 2: Off-by-one high — 4097 instead of 4096
// SHOULD FAIL
proof fn mutation_off_by_one_high() {
    assert(spec_page_index2page_ptr(1usize) == 4097usize);
}

// Test 3: Wrong base case — index 0 should map to 0, not 1
// SHOULD FAIL
proof fn mutation_wrong_base_case() {
    assert(spec_page_index2page_ptr(0usize) == 1usize);
}

// Test 4: Two distinct inputs should NOT produce the same output
// SHOULD FAIL
proof fn mutation_conflate_distinct_inputs() {
    assert(spec_page_index2page_ptr(1usize) == spec_page_index2page_ptr(2usize));
}

// Test 5: Negating the correct result — the result for index 1 IS 4096
// SHOULD FAIL
proof fn mutation_negate_correct_result() {
    assert(spec_page_index2page_ptr(1usize) != 4096usize);
}

}
