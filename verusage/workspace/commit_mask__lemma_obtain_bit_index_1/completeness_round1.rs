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
// COMPLETENESS ROUND 1: Precondition Violations
// These should all FAIL - testing that preconditions are enforced
// ============================================================

// Test R1-1: Call lemma with a=0 (violates requires a != 0)
proof fn test_r1_precondition_violation_zero() {
    let b = lemma_obtain_bit_index_1(0usize); // SHOULD FAIL
}

// Test R1-2: Call aux with a=0 (violates requires a != 0)
proof fn test_r1_aux_precondition_a_zero() {
    assert(0u64 >> 64u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(0u64, 64u64); // SHOULD FAIL
}

// Test R1-3: Call aux with hi > 64 (violates requires hi <= 64)
proof fn test_r1_aux_precondition_hi_too_large() {
    // hi=65 violates hi <= 64
    assume(1u64 >> 65u64 == 0u64);
    let i = lemma_obtain_bit_index_1_aux(1u64, 65u64); // SHOULD FAIL
}

// Test R1-4: Call aux where a >> hi != 0 (violates requires a >> hi == 0)
proof fn test_r1_aux_precondition_shift_nonzero() {
    // a=0xFF, hi=4: 0xFF >> 4 = 0xF != 0
    let i = lemma_obtain_bit_index_1_aux(0xFFu64, 4u64); // SHOULD FAIL
}

}
