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
// COMPLETENESS ROUND 3: Negated/Wrong Postconditions
// These should all FAIL
// ============================================================

// Test R3-1: Assert the bit is NOT set (negation of postcondition)
proof fn test_r3_bit_not_set(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(!is_bit_set(a, b)); // SHOULD FAIL: contradicts postcondition
}

// Test R3-2: Assert b >= 64 (contradicts b < 64)
proof fn test_r3_result_ge_64(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b >= 64); // SHOULD FAIL: contradicts postcondition
}

// Test R3-3: Assert a different bit is set (not the returned one)
proof fn test_r3_wrong_bit_set(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // Assert bit (63 - b) is set - not guaranteed
    assert(is_bit_set(a, (63 - b) as usize)); // SHOULD FAIL
}

// Test R3-4: Assert result of one call equals result of another for different inputs
proof fn test_r3_different_inputs_same_result() {
    let b1 = lemma_obtain_bit_index_1(1usize);
    let b2 = lemma_obtain_bit_index_1(0x8000_0000_0000_0000usize);
    assert(b1 == b2); // SHOULD FAIL: different bits set
}

// Test R3-5: Assert all bits are set based on one bit being set
proof fn test_r3_all_bits_set(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // Just because one bit is set doesn't mean another specific bit is set
    assert(is_bit_set(a, 0)); // SHOULD FAIL for a without bit 0
}

// Test R3-6: aux - assert negated postcondition
proof fn test_r3_aux_negated(a: u64)
    requires a != 0, a >> 64u64 == 0
{
    let i = lemma_obtain_bit_index_1_aux(a, 64);
    assert(!is_bit_set!(a, i)); // SHOULD FAIL
}

}
