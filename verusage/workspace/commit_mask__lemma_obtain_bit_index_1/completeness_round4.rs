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
// COMPLETENESS ROUND 4: Incorrect Value Assertions for Concrete Inputs
// These should all FAIL - asserting wrong specific values
// ============================================================

// Test R4-1: For a=1, assert b != 0 (wrong - bit 0 IS the only set bit)
// This tests whether spec is tight enough to determine exact bit for power-of-2
proof fn test_r4_a1_wrong_bit() {
    let b = lemma_obtain_bit_index_1(1usize);
    assert(b != 0); // SHOULD FAIL: for a=1, only bit 0 is set, so b must be 0
}

// Test R4-2: For a=2, assert b == 0 (wrong - bit 1 is set, not bit 0)
proof fn test_r4_a2_wrong_bit() {
    let b = lemma_obtain_bit_index_1(2usize);
    assert(b == 0); // SHOULD FAIL: is_bit_set(2, 0) is false
}

// Test R4-3: For a=4, assert b == 1
proof fn test_r4_a4_wrong_bit() {
    let b = lemma_obtain_bit_index_1(4usize);
    assert(b == 1); // SHOULD FAIL: is_bit_set(4, 1) is false
}

// Test R4-4: Assert b > 0 for all nonzero a (wrong for a=1)
proof fn test_r4_result_always_positive(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b > 0); // SHOULD FAIL: a=1 has b=0
}

// Test R4-5: Assert the returned bit index is the ONLY set bit
proof fn test_r4_unique_bit(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // Assert a == (1 << b), i.e., only one bit is set
    assert(a == (1usize << b)); // SHOULD FAIL: a could have multiple bits
}

// Test R4-6: For a=0x8000_0000_0000_0000, assert b < 63
proof fn test_r4_high_bit_wrong_bound() {
    let b = lemma_obtain_bit_index_1(0x8000_0000_0000_0000usize);
    assert(b < 63); // SHOULD FAIL: b must be 63
}

// Test R4-7: Assert aux returns 0 for any input
proof fn test_r4_aux_always_zero() {
    assert(0xFFu64 >> 8u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(0xFFu64, 8u64);
    assert(i == 0); // SHOULD FAIL: could return any set bit index
}

}
