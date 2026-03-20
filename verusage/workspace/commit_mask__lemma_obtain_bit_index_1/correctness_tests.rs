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
// CORRECTNESS TESTS - These should all PASS (verify successfully)
// ============================================================

// --- Parameterized Tests ---

// Test P1: Basic parameterized - any nonzero usize has a set bit index < 64
proof fn test_param_basic(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    assert(b < 64);
    assert(is_bit_set(a, b));
}

// Test P2: Parameterized - result bit index is bounded
proof fn test_param_bounded_result(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // b < 64 implies b <= 63
    assert(b <= 63);
}

// Test P3: Parameterized - calling lemma twice gives valid results both times
proof fn test_param_double_call(a: usize, c: usize)
    requires a != 0, c != 0
{
    let b1 = lemma_obtain_bit_index_1(a);
    let b2 = lemma_obtain_bit_index_1(c);
    assert(b1 < 64);
    assert(b2 < 64);
    assert(is_bit_set(a, b1));
    assert(is_bit_set(c, b2));
}

// Test P4: Parameterized - the result bit being set implies a is nonzero
proof fn test_param_set_bit_implies_nonzero(a: usize)
    requires a != 0
{
    let b = lemma_obtain_bit_index_1(a);
    // is_bit_set(a, b) means a & (1 << b) == (1 << b), so a != 0
    assert(is_bit_set(a, b));
}

// Test P5: Parameterized aux - valid inputs produce valid output
proof fn test_param_aux_basic(a: u64)
    requires a != 0, a >> 64u64 == 0
{
    let i = lemma_obtain_bit_index_1_aux(a, 64);
    assert(i < 64);
    assert(is_bit_set!(a, i));
}

// Test P6: Parameterized aux - smaller hi still works
proof fn test_param_aux_small_hi(a: u64)
    requires a != 0, a >> 32u64 == 0
{
    let i = lemma_obtain_bit_index_1_aux(a, 32);
    assert(i < 32);
    assert(is_bit_set!(a, i));
}

// --- Concrete Tests ---

// Test C1: a = 1 (only bit 0 set)
proof fn test_concrete_a_1() {
    let b = lemma_obtain_bit_index_1(1usize);
    assert(b < 64);
    assert(is_bit_set(1usize, b));
}

// Test C2: a = 2 (only bit 1 set)
proof fn test_concrete_a_2() {
    let b = lemma_obtain_bit_index_1(2usize);
    assert(b < 64);
    assert(is_bit_set(2usize, b));
}

// Test C3: a = 4 (only bit 2 set)
proof fn test_concrete_a_4() {
    let b = lemma_obtain_bit_index_1(4usize);
    assert(b < 64);
    assert(is_bit_set(4usize, b));
}

// Test C4: a = 8
proof fn test_concrete_a_8() {
    let b = lemma_obtain_bit_index_1(8usize);
    assert(b < 64);
    assert(is_bit_set(8usize, b));
}

// Test C5: a = 0xFF (low byte all set)
proof fn test_concrete_a_0xff() {
    let b = lemma_obtain_bit_index_1(0xFFusize);
    assert(b < 64);
    assert(is_bit_set(0xFFusize, b));
}

// Test C6: a = 0xFFFF
proof fn test_concrete_a_0xffff() {
    let b = lemma_obtain_bit_index_1(0xFFFFusize);
    assert(b < 64);
    assert(is_bit_set(0xFFFFusize, b));
}

// Test C7: a = 0x8000_0000_0000_0000 (highest bit)
proof fn test_concrete_a_high_bit() {
    let b = lemma_obtain_bit_index_1(0x8000_0000_0000_0000usize);
    assert(b < 64);
    assert(is_bit_set(0x8000_0000_0000_0000usize, b));
}

// Test C8: a = 0xFFFF_FFFF_FFFF_FFFF (all bits set)
proof fn test_concrete_a_max() {
    let b = lemma_obtain_bit_index_1(0xFFFF_FFFF_FFFF_FFFFusize);
    assert(b < 64);
    assert(is_bit_set(0xFFFF_FFFF_FFFF_FFFFusize, b));
}

// Test C9: a = 0x0000_0000_0000_0010 (bit 4)
proof fn test_concrete_a_bit4() {
    let b = lemma_obtain_bit_index_1(0x10usize);
    assert(b < 64);
    assert(is_bit_set(0x10usize, b));
}

// Test C10: a = 0xDEAD_BEEF (mixed pattern)
proof fn test_concrete_a_deadbeef() {
    let b = lemma_obtain_bit_index_1(0xDEAD_BEEFusize);
    assert(b < 64);
    assert(is_bit_set(0xDEAD_BEEFusize, b));
}

// Test C11: a = 0x0000_0001_0000_0000 (bit 32)
proof fn test_concrete_a_bit32() {
    let b = lemma_obtain_bit_index_1(0x1_0000_0000usize);
    assert(b < 64);
    assert(is_bit_set(0x1_0000_0000usize, b));
}

// Test C12: a = 0x4000_0000_0000_0000 (bit 62)
proof fn test_concrete_a_bit62() {
    let b = lemma_obtain_bit_index_1(0x4000_0000_0000_0000usize);
    assert(b < 64);
    assert(is_bit_set(0x4000_0000_0000_0000usize, b));
}

// --- Concrete Tests for Aux ---

// Test CA1: aux with a=1, hi=1
proof fn test_concrete_aux_a1_hi1() {
    assert(1u64 >> 1u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(1u64, 1u64);
    assert(i < 1);
    assert(is_bit_set!(1u64, i));
}

// Test CA2: aux with a=1, hi=64
proof fn test_concrete_aux_a1_hi64() {
    assert(1u64 >> 64u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(1u64, 64u64);
    assert(i < 64);
    assert(is_bit_set!(1u64, i));
}

// Test CA3: aux with a=3, hi=2
proof fn test_concrete_aux_a3_hi2() {
    assert(3u64 >> 2u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(3u64, 2u64);
    assert(i < 2);
    assert(is_bit_set!(3u64, i));
}

// Test CA4: aux with a=0xFF, hi=8
proof fn test_concrete_aux_a0xff_hi8() {
    assert(0xFFu64 >> 8u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(0xFFu64, 8u64);
    assert(i < 8);
    assert(is_bit_set!(0xFFu64, i));
}

// Test CA5: aux with a=0x8000_0000_0000_0000, hi=64
proof fn test_concrete_aux_high_bit() {
    assert(0x8000_0000_0000_0000u64 >> 64u64 == 0u64) by (bit_vector);
    let i = lemma_obtain_bit_index_1_aux(0x8000_0000_0000_0000u64, 64u64);
    assert(i < 64);
    assert(is_bit_set!(0x8000_0000_0000_0000u64, i));
}

}
