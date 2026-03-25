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
// COMPLETENESS ROUND 2: Overly Strong Postconditions
// These should all FAIL - the spec does NOT guarantee these
// ============================================================

// Test R2-1: Assert b < 32 (too strong, spec only says b < 64)
proof fn test_r2_result_less_than_32(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b < 32); // SHOULD FAIL: b could be 32..63
}

// Test R2-2: Assert b == 0 for arbitrary nonzero a
proof fn test_r2_result_always_zero(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b == 0); // SHOULD FAIL: b depends on which bit is set
}

// Test R2-3: Assert b < 8 (too strong)
proof fn test_r2_result_less_than_8(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b < 8); // SHOULD FAIL
}

// Test R2-4: Assert b is even
proof fn test_r2_result_is_even(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b % 2 == 0); // SHOULD FAIL
}

// Test R2-5: Assert specific bit index for a=0xFF
proof fn test_r2_specific_result_for_0xff() {
    let b = lemma_obtain_bit_index_1(0xFFusize);
    assert(b == 7); // SHOULD FAIL: spec says some bit is set, not which one
}

}
