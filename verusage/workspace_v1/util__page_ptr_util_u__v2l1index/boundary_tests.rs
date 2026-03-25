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

// ===== Boundary Tests =====
// Each test violates the precondition of v2l1index.
// All tests SHOULD FAIL verification.

// Test 1: va = 0 — L4 index = 0 < KERNEL_MEM_END_L4INDEX, no validity check passes
// SHOULD FAIL
fn test_boundary_va_zero() {
    let _result = v2l1index(0usize); // precondition violation: L4 index = 0
}

// Test 2: va = 1 — not aligned to any page size
// SHOULD FAIL
fn test_boundary_va_unaligned() {
    let _result = v2l1index(1usize); // precondition violation: not page-aligned
}

// Test 3: va = 0x1000 (4K aligned) but L4 index = 0
// SHOULD FAIL
fn test_boundary_4k_aligned_but_low_l4() {
    let _result = v2l1index(0x1000usize); // 4K aligned, but L4 index = 0
}

// Test 4: va = 0xFFFF_FFFF_FFFF_FFFF — upper bits set, not aligned
// SHOULD FAIL
fn test_boundary_va_max() {
    let _result = v2l1index(0xFFFF_FFFF_FFFF_FFFFusize); // upper bits set, unaligned
}

// Test 5: va = 0x0000_0080_0000_0001 — kernel space but not 4K aligned (bit 0 set)
// SHOULD FAIL
fn test_boundary_kernel_but_unaligned() {
    let _result = v2l1index(0x0000_0080_0000_0001usize); // L4 index=1 but not aligned
}

// Test 6: va = 0xFFFF_0080_0000_0000 — upper 16 bits set (outside 48-bit range)
// SHOULD FAIL
fn test_boundary_upper_bits_set() {
    let _result = v2l1index(0xFFFF_0080_0000_0000usize); // bit 63:48 != 0
}

}
