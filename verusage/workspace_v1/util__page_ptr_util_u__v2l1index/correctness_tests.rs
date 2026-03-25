use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

// ===== Definitions from target file =====

pub type L1Index = usize;

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const MEM_1g_MASK: u64 = 0x0000_fffc_0000_0000;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_4k_valid(va),
{
    unimplemented!()
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_2m_valid))]
pub fn va_2m_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_2m_valid(va),
{
    unimplemented!()
}

pub open spec fn spec_va_1g_valid(va: usize) -> bool {
    (va & (!MEM_1g_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_1g_valid))]
pub fn va_1g_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_1g_valid(va),
{
    unimplemented!()
}

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

#[verifier(when_used_as_spec(spec_v2l1index))]
pub fn v2l1index(va: usize) -> (ret: L1Index)
    requires
        va_4k_valid(va) || va_2m_valid(va) || va_1g_valid(va),
    ensures
        ret == spec_v2l1index(va),
        ret <= 0x1ff,
{
    assert((va as u64 >> 12u64 & 0x1ffu64) as usize <= 0x1ff) by (bit_vector);
    (va as u64 >> 12u64 & 0x1ffu64) as usize
}

// =============================================
// BOUNDARY TESTS — Violate preconditions
// =============================================

// Test B1: va = 0 — L4 index = 0 < KERNEL_MEM_END_L4INDEX
// SHOULD FAIL
fn test_boundary_va_zero() {
    let _result = v2l1index(0usize);
}

// Test B2: va = 1 — not page-aligned at all
// SHOULD FAIL
fn test_boundary_va_unaligned() {
    let _result = v2l1index(1usize);
}

// Test B3: va = 0x1000 — 4K aligned but L4 index = 0
// SHOULD FAIL
fn test_boundary_4k_aligned_but_low_l4() {
    let _result = v2l1index(0x1000usize);
}

// Test B4: va = usize::MAX — upper bits set, not aligned
// SHOULD FAIL
fn test_boundary_va_max() {
    let _result = v2l1index(0xFFFF_FFFF_FFFF_FFFFusize);
}

// Test B5: va in kernel space but not 4K aligned (bit 0 set)
// SHOULD FAIL
fn test_boundary_kernel_but_unaligned() {
    let _result = v2l1index(0x0000_0080_0000_0001usize);
}

// Test B6: va with upper 16 bits set (outside 48-bit address space)
// SHOULD FAIL
fn test_boundary_upper_bits_set() {
    let _result = v2l1index(0xFFFF_0080_0000_0000usize);
}

// =============================================
// BEHAVIORAL MUTATION TESTS — Wrong outputs
// =============================================

// Test M1: Assert wrong concrete L1 index value (should be 1, claim 0)
// va = 0x0000_0080_0000_1000: 4K-valid, L1 index = 1
// SHOULD FAIL
proof fn test_mutation_wrong_l1_index_value() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 0usize) by (bit_vector);
}

// Test M2: Assert result exceeds upper bound (> 0x1ff)
// va = 0x0000_0080_001f_f000: L1 index = 0x1ff (max), claim > 0x1ff
// SHOULD FAIL
proof fn test_mutation_exceeds_upper_bound() {
    let va: usize = 0x0000_0080_001f_f000usize;
    assert(spec_v2l1index(va) > 0x1ffusize) by (bit_vector);
}

// Test M3: Off-by-one — claim L1 index is 2 when actual is 1
// SHOULD FAIL
proof fn test_mutation_off_by_one() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 2usize) by (bit_vector);
}

// Test M4: Negate correctness — assert result != its own definition
// SHOULD FAIL
proof fn test_mutation_negated_correctness() {
    let va: usize = 0x0000_0080_0010_0000usize;
    let expected: usize = (va >> 12usize & 0x1ffusize) as usize;
    assert(spec_v2l1index(va) != expected) by (bit_vector);
}

// Test M5: Wrong L1 for 2M-aligned address (lower 21 bits = 0 → L1 = 0, claim 0x1ff)
// SHOULD FAIL
proof fn test_mutation_wrong_2m_l1_index() {
    let va: usize = 0x0000_0080_0020_0000usize;
    assert(spec_v2l1index(va) == 0x1ffusize) by (bit_vector);
}

// Test M6: Assert impossible value 0x200 (out of 9-bit range)
// SHOULD FAIL
proof fn test_mutation_impossible_large_value() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 0x200usize) by (bit_vector);
}

// =============================================
// LOGICAL TESTS — Unentailed properties
// =============================================

// Test L1: Assert injectivity — two addresses with same L1 bits must differ
// FALSE: distinct addresses can share L1 index
// SHOULD FAIL
proof fn test_logical_injectivity() {
    let va1: usize = 0x0000_0080_0000_1000usize;
    let va2: usize = 0x0000_0100_0000_1000usize;
    assert(spec_v2l1index(va1) != spec_v2l1index(va2)) by (bit_vector);
}

// Test L2: Assert stronger upper bound (< 256 instead of <= 511)
// FALSE: L1 index uses 9 bits
// SHOULD FAIL
proof fn test_logical_stronger_bound() {
    let va: usize = 0x0000_0080_001f_f000usize;
    assert(spec_v2l1index(va) < 256usize) by (bit_vector);
}

// Test L3: Assert 4K-valid implies 2M-valid
// FALSE: 4K addresses need not be 2M aligned
// SHOULD FAIL
proof fn test_logical_4k_implies_2m() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_va_4k_valid(va) ==> spec_va_2m_valid(va)) by (bit_vector);
}

// Test L4: Assert L1 index is always even for valid addresses
// FALSE: odd L1 indices are possible
// SHOULD FAIL
proof fn test_logical_always_even() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) % 2 == 0usize) by (bit_vector);
}

// Test L5: Assert L1 index uniquely determines the address (bijection)
// FALSE: many addresses map to the same L1 index
// SHOULD FAIL
proof fn test_logical_l1_determines_va() {
    let va1: usize = 0x0000_0080_0000_1000usize;
    let va2: usize = 0x0000_0080_0020_1000usize;
    assert(spec_v2l1index(va1) == spec_v2l1index(va2) ==> va1 == va2) by (bit_vector);
}

// Test L6: Assert L1 index is always 0
// FALSE: non-zero L1 indices exist for 4K addresses
// SHOULD FAIL
proof fn test_logical_always_zero() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 0usize) by (bit_vector);
}

}
