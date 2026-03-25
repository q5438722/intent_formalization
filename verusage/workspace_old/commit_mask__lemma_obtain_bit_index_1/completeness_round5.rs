use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

spec fn is_bit_set(a: usize, b: usize) -> bool {
    a & (1usize << b) == (1usize << b)
}

#[allow(unused_macros)]
macro_rules! is_bit_set {
    ($a:expr, $b:expr) => {
        $a & (1u64 << $b) == (1u64 << $b)
    }
}

#[verifier::external_body]
proof fn lemma_obtain_bit_index_1_aux(a: u64, hi: u64) -> (i: u64)
    requires
        a != 0,
        hi <= 64,
        a >> hi == 0,
    ensures
        i < hi,
        is_bit_set!(a, i),
    decreases hi
{
    unimplemented!()
}

proof fn lemma_obtain_bit_index_1(a: usize) -> (b: usize)
    requires a != 0
    ensures
        b < 64,
        is_bit_set(a, b)
{
    reveal(is_bit_set);
    assert(a as u64 >> 64 == 0) by (bit_vector);
    lemma_obtain_bit_index_1_aux(a as u64, 64) as usize
}

pub struct CommitMask {
    mask: [usize; 8],
}

// ============================================================
// COMPLETENESS ROUND 5: Cross-function and Edge Case Violations
// These should all FAIL
// ============================================================

// Test R5-1: Assert is_bit_set for 0 (no bits are set in 0)
proof fn test_r5_zero_has_set_bit() {
    // Try to prove 0 has a set bit - this is false
    assert(is_bit_set(0usize, 0usize)); // SHOULD FAIL
}

// Test R5-2: Assert is_bit_set beyond valid range
proof fn test_r5_bit_beyond_range() {
    // Bit 64 is out of range for u64
    assert(is_bit_set(0xFFFF_FFFF_FFFF_FFFFusize, 64usize)); // SHOULD FAIL
}

// Test R5-3: Use lemma result to claim a stronger fact about a
proof fn test_r5_stronger_fact_about_input(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // Knowing one bit is set doesn't tell us a >= (1 << b + 1)
    assert(a > (1usize << b)); // SHOULD FAIL: a could be exactly (1 << b)
}

// Test R5-4: Assert two calls on same input return same index
proof fn test_r5_deterministic_result(a: usize)
    requires a != 0
{
    let b1 = lemma_obtain_bit_index_1(a);
    let b2 = lemma_obtain_bit_index_1(a);
    assert(b1 == b2); // SHOULD FAIL: spec doesn't guarantee determinism
}

// Test R5-5: Assert aux postcondition with wrong bit position
proof fn test_r5_aux_wrong_bit() {
    assert(1u64 >> 64u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(1u64, 64u64);
    // We know i < 64 and is_bit_set(1, i)
    // For a=1, is_bit_set(1, i) means i == 0
    // Try to assert i == 1 -- should fail
    assert(i == 1); // SHOULD FAIL
}

// Test R5-6: Assert that a has ALL bits set below b
proof fn test_r5_all_lower_bits_set(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // Spec only says bit b is set, not that all lower bits are set
    assert(a & (((1usize << b) - 1) as usize) == ((1usize << b) - 1) as usize); // SHOULD FAIL
}

// Test R5-7: Assert aux with hi=0 works (impossible - contradicts a != 0 and a >> 0 == 0)
proof fn test_r5_aux_hi_zero() {
    // a=1, hi=0: 1 >> 0 = 1 != 0, so precondition a >> hi == 0 is violated
    let i = lemma_obtain_bit_index_1_aux(1u64, 0u64); // SHOULD FAIL
}

// Test R5-8: Cross-function - use aux result as input to main lemma incorrectly
proof fn test_r5_cross_function_misuse() {
    assert(5u64 >> 3u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(5u64, 3u64);
    // i < 3, try to use i as input to lemma_obtain_bit_index_1
    // but i as usize might be 0, which violates requires a != 0
    let b = lemma_obtain_bit_index_1(i as usize); // SHOULD FAIL: i could be 0
}

}
